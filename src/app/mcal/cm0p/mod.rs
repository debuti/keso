#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;

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
    systick: SysTick_Type,
    nvic: NVIC_Type,
    scb: SCB_Type,
    mpu: MPU_Type,
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

    fn check_irq_param(num: u32) {
        assert_eq!(false, num >= super::NUM_IRQS);
    }

    #[inline(never)]
    pub fn irq_is_enabled(&self, num: u32) -> bool {
        Self::check_irq_param(num);
        (self.nvic.iser.read() & (1 << num)) != 0
    }

    #[inline(never)]
    pub fn irq_set_enabled(&mut self, num: u32, enabled: bool) {
        Self::check_irq_param(num);
        self.irq_set_mask_enabled(1 << num, enabled);
    }
    
    #[inline(never)]
    pub fn irq_set_mask_enabled(&mut self, mask: u32, enabled: bool) {
        if enabled {
            // Clear pending before enable
            // (if IRQ is actually asserted, it will immediately re-pend)
            self.nvic.icpr.write(mask);
            self.nvic.iser.write(mask);
        } else {
            self.nvic.icer.write(mask);
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