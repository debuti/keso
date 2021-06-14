#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;

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
    pub(crate) const unsafe fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub const PTR: *mut self::RegisterBlock = super::SIO_BASE as *mut _;


    /* \brief Check the read FIFO to see if there is data waiting
    *  \ingroup multicore_fifo
    *
    *  \return true if the FIFO has data in it, false otherwise
    */
    #[inline(never)]
    fn multicore_fifo_rvalid(&self) -> bool {
        // Value is 1 if this core’s RX FIFO is not empty
        self.fifo_st.read() & defs::SIO_FIFO_ST_VLD_BITS != 0
    }

    /* \brief Check the write FIFO to see if it is ready for more data
    *  \ingroup multicore_fifo
    *
    *  @return true if the FIFO has room for more data, false otherwise
    */
    #[inline(never)]
    fn multicore_fifo_wready(&self) -> bool {
        // Value is 1 if this core’s TX FIFO is not full
        self.fifo_st.read() & defs::SIO_FIFO_ST_RDY_BITS != 0
    }

    /* \brief Pop data from the FIFO.
    *  \ingroup multicore_fifo
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
    *  \ingroup multicore_fifo
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
    *  \ingroup multicore_fifo
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
        unsafe {
            let cmd_sequence = [0, 0, 1, *vt, *sp, *entry];

            let mut cm0p = super::cm0p::Peripheral::new();

            let enabled = cm0p.irq_is_enabled(super::SIO_IRQ_PROC0);
            cm0p.irq_set_enabled(super::SIO_IRQ_PROC0, false);
        
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
                if seq < cmd_sequence.len() {break;}
            }
        
            cm0p.irq_set_enabled(super::SIO_IRQ_PROC0, enabled);
        }
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