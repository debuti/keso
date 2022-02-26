#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;

#[derive(Copy, Clone)]
pub enum IrqId {
    _Reserved0 = 0,
    Reset = 1,
    NMI = 2,
    HardFault = 3,
    _Reserved4 = 4,
    _Reserved5 = 5,
    _Reserved6 = 6,
    _Reserved7 = 7,
    _Reserved8 = 8,
    _Reserved9 = 9,
    _Reserved10 = 10,
    Svc = 11,
    _Reserved12 = 12,
    _Reserved13 = 13,
    PendSV = 14,
    SysTick = 15,
    Timer0 = 16,  // RP2040 specific exceptions start here
    Timer1 = 17,
    Timer2 = 18,
    Timer3 = 19,
    PwmWrap = 20,
    UsbCtrl = 21,
    Xip = 22,
    Pio0_0 = 23,
    Pio0_1 = 24,
    Pio1_0 = 25,
    Pio1_1 = 26,
    Dma0 = 27,
    Dma1 = 28,
    IoBank0 = 29,
    IoQspi = 30,
    SioProc0 = 31,
    SioProc1 = 32,
    Clocks = 33,
    Spi0 = 34,
    Spi1 = 35,
    Uart0 = 36,
    Uart1 = 37,
    AdcFifo = 38,
    I2C0 = 39,
    I2C1 = 40,
    Rtc = 41,
}

/**
  \brief  Structure type to access the System Timer (SysTick).
 */
#[repr(C)]
pub struct SysTick_Type {
    csr: Volatile<u32>,                    /* Offset: 0x000 (R/W)  SysTick Control and Status Register */
    rvr: Volatile<u32>,                    /* Offset: 0x004 (R/W)  SysTick Reload Value Register */
    cvr: Volatile<u32>,                    /* Offset: 0x008 (R/W)  SysTick Current Value Register */
    calib: Volatile<u32>,                  /* Offset: 0x00C (R/ )  SysTick Calibration Register */
}

/**
  \brief  Structure type to access the Nested Vectored Interrupt Controller (NVIC).
 */
#[repr(C)]
pub struct NVIC_Type {
    iser: Volatile<u32>,                   /* Offset: 0x000 (R/W)  Interrupt Set Enable Register */
    _reserved0: [Volatile<u32>;31],
    icer: Volatile<u32>,                   /* Offset: 0x080 (R/W)  Interrupt Clear Enable Register */
    _reserved1: [Volatile<u32>;31],
    ispr: Volatile<u32>,                   /* Offset: 0x100 (R/W)  Interrupt Set Pending Register */
    _reserved2: [Volatile<u32>;31],
    icpr: Volatile<u32>,                   /* Offset: 0x180 (R/W)  Interrupt Clear Pending Register */
    _reserved3: [Volatile<u32>;31],
    _reserved4: [Volatile<u32>;64],
    ip: [Volatile<u32>;8],                 /* Offset: 0x300 (R/W)  Interrupt Priority Register */
} 

/**
  \brief  Structure type to access the System Control Block (SCB).
 */
#[repr(C)]
pub struct SCB_Type {
    cpuid: Volatile<u32>,                  /* Offset: 0x000 (R/ )  CPUID Base Register */
    icsr: Volatile<u32>,                   /* Offset: 0x004 (R/W)  Interrupt Control and State Register */
    vtor: Volatile<u32>,                   /* Offset: 0x008 (R/W)  Vector Table Offset Register */
    aircr: Volatile<u32>,                  /* Offset: 0x00C (R/W)  Application Interrupt and Reset Control Register */
    scr: Volatile<u32>,                    /* Offset: 0x010 (R/W)  System Control Register */
    ccr: Volatile<u32>,                    /* Offset: 0x014 (R/W)  Configuration Control Register */
    _reserved0: Volatile<u32>,
    shc: [Volatile<u32>;2],                /* Offset: 0x01C (R/W)  System Handlers Priority Registers. [0] is RESERVED */
    shcsr: Volatile<u32>,                  /* Offset: 0x024 (R/W)  System Handler Control and State Register */
 }

 /**
  \brief  Structure type to access the Memory Protection Unit (MPU).
 */
#[repr(C)]
pub struct MPU_Type {
    r#type: Volatile<u32>,                 /* Offset: 0x000 (R/ )  MPU Type Register */
    ctrl: Volatile<u32>,                   /* Offset: 0x004 (R/W)  MPU Control Register */
    rnr: Volatile<u32>,                    /* Offset: 0x008 (R/W)  MPU Region RNRber Register */
    rbar: Volatile<u32>,                   /* Offset: 0x00C (R/W)  MPU Region Base Address Register */
    rasr: Volatile<u32>,                   /* Offset: 0x010 (R/W)  MPU Region Attribute and Size Register */
}


#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [Volatile<u8>;0xe010],
    systick: SysTick_Type,                 /* Offset: 0xe010 */
    _reserved1: [Volatile<u8>;0xe100 - core::mem::size_of::<SysTick_Type>() - 0xe010],
    nvic: NVIC_Type,                       /* Offset: 0xe100 */
    _reserved2: [Volatile<u8>;0xed00 - core::mem::size_of::<NVIC_Type>() - 0xe100],
    scb: SCB_Type,                         /* Offset: 0xed00 */
    _reserved3: [Volatile<u8>;0xed90 - core::mem::size_of::<SCB_Type>() - 0xed00],
    mpu: MPU_Type,                         /* Offset: 0xed90 */
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

    pub const PTR: *mut self::RegisterBlock = super::PPB_BASE as *mut _;
    
    fn check_irq_param(irq: IrqId) {
        assert!((irq as u32) < super::NUM_IRQS);
    }

    #[inline(never)]
    pub fn irq_is_enabled(&self, irq: IrqId) -> bool {
        Self::check_irq_param(irq);
        (self.nvic.iser.read() & (1 << (irq as u32 - 16))) != 0
    }

    #[inline(never)]
    pub fn irq_set_enabled(&mut self, irq: IrqId, enabled: bool) {
//        crate::app::mcal::timer::Peripheral::delay_nops(u32::MAX);
        Self::check_irq_param(irq);
        self.irq_set_mask_enabled(1 << (irq as u32 - 16), enabled);
    }
    
    #[inline(never)]
    pub fn irq_set_mask_enabled(&mut self, mask: u32, enabled: bool) {
        // Clear pending before enable (W 1 to set to not pending)
        // (if IRQ is actually asserted, it will immediately re-pend)
        self.nvic.icpr.write(mask);
        if enabled {
            // Interrupt set enable (W 1 to enable the interrupt)
            self.nvic.iser.write(mask);
        } else {
            // Interrupt clear enable (W 1 to disable the interrupt)
            self.nvic.icer.write(mask);
        }
    }

    #[inline(never)]
    pub fn irq_set_pending(&mut self, irq: IrqId, pending: bool) {
        Self::check_irq_param(irq);
        self.irq_set_mask_enabled(1 << (irq as u32 - 16), pending);
    }
    
    #[inline(never)]
    pub fn irq_set_mask_pending(&mut self, mask: u32, pending: bool) {
        if pending {
            self.nvic.ispr.write(mask);
        }
        else {
            self.nvic.icpr.write(mask);
        }        
    }
    
    #[inline(never)]
    pub fn irq_set_exclusive_handler(&mut self, irq: IrqId, handler: unsafe extern "C" fn()) {
        Self::check_irq_param(irq);
        unsafe {
            let mut sio = super::sio::Peripheral::new();
            let saved = sio.spin_lock_blocking(super::sio::SpinlockID::Irq as usize);
            // update vtable (vtable_handler may be same or updated depending on cases, but we do it anyway for compactness)
            self.irq_set_vtable_handler(irq, handler);
            super::intrinsics::dmb();
            sio.spin_unlock(super::sio::SpinlockID::Irq as usize, saved);
        }
    }

    #[inline(never)]
    pub fn irq_reset_handler(&mut self, irq: IrqId) {
        self.irq_set_exclusive_handler(irq, crate::nohandler);
    }

    #[inline(never)]
    fn irq_get_vtable_handler(&self, irq: IrqId) -> fn() {
        Self::check_irq_param(irq);
        unsafe {
          let vt = self.scb.vtor.read() as *const fn();
          let vtable = core::slice::from_raw_parts(vt, 48);
          vtable[16 + irq as usize]
        }
    }

    #[inline(never)]
    fn irq_set_vtable_handler(&self, irq: IrqId, handler: unsafe extern "C" fn()) {
        Self::check_irq_param(irq);
        unsafe {
          let vt = self.scb.vtor.read() as *mut unsafe extern "C" fn();
          let vtable = core::slice::from_raw_parts_mut(vt, 48);
          vtable[irq as usize] = handler;
        }
    }
    
    #[inline(never)]
    pub fn save_and_disable_interrupts() -> u32 {
        let status = super::intrinsics::getprimask();
        super::intrinsics::disirq();
        status
    }
    
    #[inline(never)]
    pub fn restore_interrupts(irq:u32){
        super::intrinsics::enairq(irq);
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