#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;


/** \file hardware/sync.h
 *  \defgroup hardware_sync hardware_sync
 *
 * Low level hardware spin locks, barrier and processor event APIs
 *
 * Spin Locks
 * ----------
 *
 * The RP2040 provides 32 hardware spin locks, which can be used to manage mutually-exclusive access to shared software
 * and hardware resources.
 *
 * Generally each spin lock itself is a shared resource,
 * i.e. the same hardware spin lock can be used by multiple higher level primitives (as long as the spin locks are neither held for long periods, nor
 * held concurrently with other spin locks by the same core - which could lead to deadlock). A hardware spin lock that is exclusively owned can be used
 * individually without more flexibility and without regard to other software. Note that no hardware spin lock may
 * be acquired re-entrantly (i.e. hardware spin locks are not on their own safe for use by both thread code and IRQs) however the default spinlock related
 * methods here (e.g. \ref spin_lock_blocking) always disable interrupts while the lock is held as use by IRQ handlers and user code is common/desirable,
 * and spin locks are only expected to be held for brief periods.
 *
 * The SDK uses the following default spin lock assignments, classifying which spin locks are reserved for exclusive/special purposes
 * vs those suitable for more general shared use:
 *
 * Number (ID) | Description
 * :---------: | -----------
 * 0-13        | Currently reserved for exclusive use by the SDK and other libraries. If you use these spin locks, you risk breaking SDK or other library functionality. Each reserved spin lock used individually has its own PICO_SPINLOCK_ID so you can search for those.
 * 14,15       | (\ref PICO_SPINLOCK_ID_OS1 and \ref PICO_SPINLOCK_ID_OS2). Currently reserved for exclusive use by an operating system (or other system level software) co-existing with the SDK.
 * 16-23       | (\ref PICO_SPINLOCK_ID_STRIPED_FIRST - \ref PICO_SPINLOCK_ID_STRIPED_LAST). Spin locks from this range are assigned in a round-robin fashion via \ref next_striped_spin_lock_num(). These spin locks are shared, but assigning numbers from a range reduces the probability that two higher level locking primitives using _striped_ spin locks will actually be using the same spin lock.
 * 24-31       | (\ref PICO_SPINLOCK_ID_CLAIM_FREE_FIRST - \ref PICO_SPINLOCK_ID_CLAIM_FREE_LAST). These are reserved for exclusive use and are allocated on a first come first served basis at runtime via \ref spin_lock_claim_unused()
 */

 pub enum SpinlockID {
    Irq = 9,
    Timer = 10,
    HardwareClaim = 11,
    Os1 = 14,
    Os2 = 15,
    Striped0 = 16,
    Striped1 = 17,
    Striped2 = 18,
    Striped3 = 19,
    Striped4 = 20,
    Striped5 = 21,
    Striped6 = 22,
    Striped7 = 23,
    ClaimFree0 = 24,
    ClaimFree1 = 25,
    ClaimFree2 = 26,
    ClaimFree3 = 27,
    ClaimFree4 = 28,
    ClaimFree5 = 29,
    ClaimFree6 = 30,
    ClaimFree7 = 31,
}

#[repr(C)]
pub struct interp_hw {
    accum: [Volatile<u32>; 2],
    base: [Volatile<u32>; 3],
    pop: [Volatile<u32>; 3],
    peek: [Volatile<u32>; 3],
    ctrl: [Volatile<u32>; 2],
    add_raw: [Volatile<u32>; 2],
    base01: Volatile<u32>,
}

#[repr(C)]
pub struct RegisterBlock {
    cpuid: Volatile<u32>,
    gpio_in: Volatile<u32>,
    gpio_hi_in: Volatile<u32>,
    _pad: [u8; 0x4],

    gpio_out: Volatile<u32>,
    gpio_set: Volatile<u32>,
    gpio_clr: Volatile<u32>,
    gpio_togl: Volatile<u32>,

    gpio_oe: Volatile<u32>,
    gpio_oe_set: Volatile<u32>,
    gpio_oe_clr: Volatile<u32>,
    gpio_oe_togl: Volatile<u32>,

    gpio_hi_out: Volatile<u32>,
    gpio_hi_set: Volatile<u32>,
    gpio_hi_clr: Volatile<u32>,
    gpio_hi_togl: Volatile<u32>,

    gpio_hi_oe: Volatile<u32>,
    gpio_hi_oe_set: Volatile<u32>,
    gpio_hi_oe_clr: Volatile<u32>,
    gpio_hi_oe_togl: Volatile<u32>,

    // Each FIFO is 32 bits wide, and eight entries deep
    fifo_st: Volatile<u32>,
    fifo_wr: Volatile<u32>,
    fifo_rd: Volatile<u32>,
    spinlock_st: Volatile<u32>,

    div_udividend: Volatile<u32>,
    div_udivisor: Volatile<u32>,
    div_sdividend: Volatile<u32>,
    div_sdivisor: Volatile<u32>,

    div_quotient: Volatile<u32>,
    div_remainder: Volatile<u32>,
    div_csr: Volatile<u32>,

    _pad2: [u8; 0x4],

    interp: [interp_hw; 2],

    spinlock: [Volatile<u32>; 32],
}

pub struct Peripheral {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for Peripheral {}

impl Peripheral {
    #[inline(always)]
    pub(crate) const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub const PTR: *mut self::RegisterBlock = super::SIO_BASE as *mut _;

    /* \brief Get the current core number
    *
    * \return The core number the call was made from
    */
    #[inline(never)]
    pub fn get_core_num(&self) -> u32 {
        return self.cpuid.read();
    }

    /* \brief Check the read FIFO to see if there is data waiting
    *
    *  \return true if the FIFO has data in it, false otherwise
    */
    #[inline(never)]
    fn multicore_fifo_rvalid(&self) -> bool {
        // Value is 1 if this core’s RX FIFO is not empty
        self.fifo_st.read() & defs::SIO_FIFO_ST_VLD_BITS != 0
    }

    /* \brief Check the write FIFO to see if it is ready for more data
    *
    *  @return true if the FIFO has room for more data, false otherwise
    */
    #[inline(never)]
    fn multicore_fifo_wready(&self) -> bool {
        // Value is 1 if this core’s TX FIFO is not full
        self.fifo_st.read() & defs::SIO_FIFO_ST_RDY_BITS != 0
    }

    /* \brief Pop data from the FIFO.
    *
    * This function will block until there is data ready to be read
    * Use multicore_fifo_rvalid() to check if data is ready to be read if you don't
    * want to block.
    *
    * \return 32 bit unsigned data from the FIFO.
    */
    #[inline(never)]
    fn multicore_fifo_pop_blocking(&self) -> u32 {
        // If nothing there yet, we wait for an event first,
        // to try and avoid too much busy waiting
        while !self.multicore_fifo_rvalid()
         {super::intrinsics::wfe();}

        self.fifo_rd.read()
    }

    /* \brief Push data on to the FIFO.
    *
    * This function will block until there is space for the data to be sent.
    * Use multicore_fifo_wready() to check if it is possible to write to the
    * FIFO if you don't want to block.
    *
    * \param data A 32 bit value to push on to the FIFO
    */
    #[inline(never)]
    fn multicore_fifo_push_blocking(&mut self, data: u32) {
        // We wait for the fifo to have some space
        while !self.multicore_fifo_wready() {/*NOP*/}

        self.fifo_wr.write(data);

        // Fire off an event to the other core
        super::intrinsics::sev();
    }

    /* \brief Flush any data in the incoming FIFO
    *
    */
    #[inline(never)]
    fn multicore_fifo_drain(&self) {
        while self.multicore_fifo_rvalid() {
            self.fifo_rd.read();
        }
    }

    #[inline(never)]
    pub fn launch_core1(&mut self, entry: *const usize, sp: *const usize, vt: *const usize) {
        let cmd_sequence = [0, 0, 1, vt as usize, sp as usize, entry as usize];

        let mut cm0p = super::cm0p::Peripheral::new();

        let enabled = cm0p.irq_is_enabled(super::cm0p::IrqId::SioProc0);
        cm0p.irq_set_enabled(super::cm0p::IrqId::SioProc0, false);
    
        let mut seq = 0;
        loop {
            let cmd = cmd_sequence[seq] as u32;
            // we drain before sending a 0
            if cmd == 0 {
                self.multicore_fifo_drain();
                super::intrinsics::sev(); // core 1 may be waiting for fifo space
            }
            self.multicore_fifo_push_blocking(cmd);
            let response = self.multicore_fifo_pop_blocking();
            
            // move to next state on correct response otherwise start over
            seq = if cmd == response {seq + 1} else {0};
            if seq == cmd_sequence.len() {break;}
        }
    
        cm0p.irq_set_enabled(super::cm0p::IrqId::SioProc0, enabled);
    }
    
    #[inline(never)]
    pub fn spin_lock_blocking(&mut self, lock:usize) -> u32 {
        let save = super::cm0p::Peripheral::save_and_disable_interrupts();
        // Note we don't do a wfe or anything, because by convention these spin_locks are VERY SHORT LIVED and NEVER BLOCK and run
        // with INTERRUPTS disabled (to ensure that)... therefore nothing on our core could be blocking us, so we just need to wait on another core
        // anyway which should be finished soon
        loop {
            if self.spinlock[lock].read() != 0 {break;}
        }
        super::intrinsics::dmb();
        return save;
    }
   
    #[inline(never)]
    pub fn spin_unlock(&mut self, lock:usize, irq:u32) {
        super::intrinsics::dmb();
        self.spinlock[lock].write(0);
        super::cm0p::Peripheral::restore_interrupts(irq);
    }

}

impl ops::Deref for Peripheral {
    type Target = self::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl ops::DerefMut for Peripheral {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *Self::PTR }
    }
}