pub mod mcal;
mod tasks;

pub const PICO_DEFAULT_LED_PIN: usize = 25;


#[inline(never)]
fn reset_setup() {
  unsafe {
    let mut resets = mcal::resets::Peripheral::new();
    resets.enable(mcal::resets::ResetDevice::IoBank0);
    resets.enable(mcal::resets::ResetDevice::PadsBank0);
  }
}

#[inline(never)]
fn gpio_setup() {
  unsafe {
    let mut a = mcal::padsbank0::Peripheral::new();
    a.writeio(PICO_DEFAULT_LED_PIN, 0x56);
  }
}

#[inline(never)]
fn multicore_setup() {
  extern "C" {
    // These symbols come from `memory.ld`
    static _reset_1: usize;
    static _stack1_start: usize;
    static _vector_table_1: usize;
  }
  unsafe {
    let reset_1: *const usize = _reset_1 as *const usize;
    let stack1_start: *const usize = &_stack1_start;
    let vector_table_1: *const usize = &_vector_table_1;
    let mut sio = mcal::sio::Peripheral::new();
    sio.launch_core1(reset_1, stack1_start ,vector_table_1);
  }
}

const UART_TX_PIN: usize = 0;
const UART_RX_PIN: usize = 1;

#[inline(never)]
fn uart_setup() {
  unsafe {
    // Set up our UART with the required speed.
    let mut uart = mcal::uart::Peripheral::new(mcal::uart::Uart::Uart0);
    let _ = uart.uart_init(115200);
        
    // Set the TX and RX pins by using the function select on the GPIO
    // Set datasheet for more information on function select
    let mut iobank0 = mcal::iobank0::Peripheral::new();
    iobank0.gpio_set_function(UART_TX_PIN, mcal::iobank0::GpioFunction::UART);
    iobank0.gpio_set_function(UART_RX_PIN, mcal::iobank0::GpioFunction::UART);
  }
}

pub fn one_time_init() {
  reset_setup();
  gpio_setup();
  uart_setup();
  //multicore_setup();
}

#[inline(never)]
pub fn c0(schedtable : tasks::SchedTable) -> ! {
  // Do the required initializations here
  one_time_init();
  
  // Start scheduler in core0
  schedtable.start();
}


#[inline(never)]
pub fn c1(schedtable : tasks::SchedTable) -> ! {

  // Start scheduler in core1
  schedtable.start();
}

#[inline(never)]
pub fn taskled() {  
  static mut LEDST: bool = false;
  unsafe {
    let mut iobank0 = mcal::iobank0::Peripheral::new();
    if LEDST {
      iobank0.force_low(PICO_DEFAULT_LED_PIN);
    }
    else {
      iobank0.force_high(PICO_DEFAULT_LED_PIN);
    }
    LEDST = !LEDST;
    //mcal::timer::Peripheral::delay_nops(1000000000); 
  }
}

#[inline(never)]
pub fn tasknop() {
  //NOP FOREVER!
  mcal::timer::Peripheral::delay_nops(4294967295);   
}

#[inline(never)]
pub fn taskuart() {
  unsafe {
    let mut uart = mcal::uart::Peripheral::new(mcal::uart::Uart::Uart0);
    //mcal::timer::Peripheral::delay_nops(10000000);
    uart.puts("Hello, keso!\n");
  }
}

#[inline(never)]
pub fn main() -> ! {
  unsafe {
    if mcal::sio::Peripheral::new().get_core_num() == 0 {
      c0(tasks::SchedTable { macroperiod : 1_000_000,
                             schedpoints : &mut[(0,       tasks::Task::new("c0t0", taskled, &mut [0xCAFECAFE; 512], 512, 0)),
                                                (500_000, tasks::Task::new("c0t1", tasknop, &mut [0xCAFECAFE; 512], 512, 0)),
                                               ],
                           });
    }
    else {
      c1(tasks::SchedTable { macroperiod : 10_000_000,
                             schedpoints : &mut[(0,       tasks::Task::new("c1t0", taskuart, &mut [0xCAFECAFE; 512], 512, 0)),
                                               ],
                           });
    }
  }
}
