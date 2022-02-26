pub mod mcal;
mod tasks;

pub const PICO_DEFAULT_LED_PIN: usize = 25;

const UART_TX_PIN: usize = 0;
const UART_RX_PIN: usize = 1;

#[inline(never)]
fn reset_setup() {
  let mut resets = mcal::resets::Peripheral::new();
  resets.enable(mcal::resets::ResetDevice::IoBank0);
  resets.enable(mcal::resets::ResetDevice::PadsBank0);
}

#[inline(never)]
fn gpio_setup() {
  let mut pads = mcal::padsbank0::Peripheral::new();
  pads.writeio(PICO_DEFAULT_LED_PIN, 0x56);
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
    sio.launch_core1(reset_1, stack1_start, vector_table_1);
  }
}

#[inline(never)]
fn uart_setup() {
  // Set up our UART with the required speed.
  let mut uart = mcal::uart::Peripheral::new(mcal::uart::Uart::Uart0);
  let _ = uart.uart_init(115200);
      
  // Set the TX and RX pins by using the function select on the GPIO
  // Set datasheet for more information on function select
  let mut iobank0 = mcal::iobank0::Peripheral::new();
  iobank0.gpio_set_function(UART_TX_PIN, mcal::iobank0::GpioFunction::UART);
  iobank0.gpio_set_function(UART_RX_PIN, mcal::iobank0::GpioFunction::UART);
}

pub fn one_time_init() {
  reset_setup();
  gpio_setup();
  uart_setup();
  multicore_setup();
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
pub fn taskledon() {
  let mut iobank0 = mcal::iobank0::Peripheral::new();
  loop {
    iobank0.force_high(PICO_DEFAULT_LED_PIN);
    tasks::Task::finishjob();
  }
}

#[inline(never)]
pub fn taskledoff() {
  let mut iobank0 = mcal::iobank0::Peripheral::new();
  loop {
    iobank0.force_low(PICO_DEFAULT_LED_PIN);
    tasks::Task::finishjob();
  }
}

#[inline(never)]
pub fn tasknop() {
  loop {
    mcal::timer::Peripheral::delay_nops(100000);
    tasks::Task::finishjob();   
  }
}

#[inline(never)]
pub fn taskuart_sayhello() {
  let mut uart = mcal::uart::Peripheral::new(mcal::uart::Uart::Uart0);
  loop {
    uart.puts("Hello, ");
    tasks::Task::finishjob();
  }
}

#[inline(never)]
pub fn taskuart_saykeso() {
  let mut uart = mcal::uart::Peripheral::new(mcal::uart::Uart::Uart0);
  loop {
    uart.puts("keso!\n\r");
    tasks::Task::finishjob();
  }
}

#[inline(never)]
pub fn main() -> ! {
  if mcal::sio::Peripheral::new().get_core_num() == 0 {
    c0(tasks::SchedTable { macroperiod : 1_000_000,
                            tasks       : &mut [tasks::Task::new("c0t0",  taskledon, &mut [0xCAFECAFE; 256], 256, 0),
                                                tasks::Task::new("c0t1", taskledoff, &mut [0xDEADBEEF; 256], 256, 0),
                                              ],
                            schedpoints : &[(0,       0), /* Start tasks[0] at 0 us */
                                            (500_000, 1), /* Start tasks[1] at 0.5 s */
                                          ],
                          });
  }
  else {
    c1(tasks::SchedTable { macroperiod : 10_000_000,
                            tasks       : &mut [tasks::Task::new("c1t0", taskuart_sayhello, &mut [0xDAD0D1D0; 256], 256, 0),
                                                tasks::Task::new("c1t1",           tasknop, &mut [0xBEEFFEED; 256], 256, 0),
                                                tasks::Task::new("c1t2",  taskuart_saykeso, &mut [0xc0c0c4c4; 256], 256, 0),
                                              ],
                            schedpoints : &[(0,         0),
                                            (1_000_000, 1),
                                            (5_000_000, 2),
                                            (6_000_000, 1),
                                          ],
                          });
  }
}
