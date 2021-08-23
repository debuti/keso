pub mod mcal;

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
    static _vector_table_1: usize; //0x
  }
  unsafe {
    let reset_1: *const usize = _reset_1 as *const usize;
    let stack1_start: *const usize = &_stack1_start;
    let vector_table_1: *const usize = &_vector_table_1;
    let mut sio = mcal::sio::Peripheral::new();
    sio.launch_core1(reset_1, stack1_start ,vector_table_1);
  }
}

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

const UART_TX_PIN: usize = 0;
const UART_RX_PIN: usize = 1;

const DELAY: u32 = 1_000_000;
const ALARM: mcal::timer::AlarmId = mcal::timer::AlarmId::Alarm0;
static mut LEDST: bool = false;

#[inline(never)]
pub fn c0() -> ! {
  reset_setup();
  gpio_setup();
  multicore_setup();
  unsafe {
    let mut timer = mcal::timer::Peripheral::new();
    timer.set_alarm_relative(ALARM, DELAY, timerhandleron); 
  }
  loop {}
}

#[no_mangle]
#[inline(never)]
pub fn timerhandleron() {
  unsafe {
    let mut timer = mcal::timer::Peripheral::new();

    timer.clear_alarm(ALARM);
    
    let mut iobank0 = mcal::iobank0::Peripheral::new();
    iobank0.force_high(PICO_DEFAULT_LED_PIN);
    
    //mcal::timer::Peripheral::delay_nops(1000000000);
    timer.set_alarm_relative(ALARM, DELAY, timerhandleroff); 
  }
}

#[no_mangle]
#[inline(never)]
pub fn timerhandleroff() {
  unsafe {
    let mut timer = mcal::timer::Peripheral::new();

    timer.clear_alarm(ALARM);
    
    let mut iobank0 = mcal::iobank0::Peripheral::new();
    iobank0.force_low(PICO_DEFAULT_LED_PIN);
 
    timer.set_alarm_relative(ALARM, DELAY, timerhandleron); 
  }
}

#[inline(never)]
pub fn c1() -> ! {
  uart_setup();
  unsafe {
    let mut uart = mcal::uart::Peripheral::new(mcal::uart::Uart::Uart0);
    loop {
        mcal::timer::Peripheral::delay_nops(10000000);
        uart.puts("Hello, keso!\n");
    }
  }
}

#[inline(never)]
pub fn main() -> ! {
  unsafe {
    let sio = mcal::sio::Peripheral::new();
    if sio.get_core_num() == 0 {
      c0();
    }
    else {
      c1();
    }
  }
}
