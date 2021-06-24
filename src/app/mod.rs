pub mod mcal;

pub const PICO_DEFAULT_LED_PIN: usize = 25;

 #[inline(never)]
pub fn delay(ticks: usize) {
    for _ in 0..ticks {
        mcal::intrinsics::nop();
    }
}

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
    // These symbols come from `linker.ld`
    static _reset_1: *const usize;
    static _stack1_start: *const usize;
    static _vector_table_1: *const usize;
  }
  unsafe {
    let mut sio = mcal::sio::Peripheral::new();
    sio.launch_core1(_reset_1, _stack1_start, _vector_table_1);
  }
}

#[inline(never)]
fn uart_setup() {
  unsafe {
    // Set up our UART with the required speed.
    let mut uart = mcal::uart::Peripheral::new(mcal::uart::Uart::Uart0);
    uart.uart_init(115200);
        
    // Set the TX and RX pins by using the function select on the GPIO
    // Set datasheet for more information on function select
    let mut iobank0 = mcal::iobank0::Peripheral::new();
    iobank0.gpio_set_function(UART_TX_PIN, mcal::iobank0::GpioFunction::UART);
    iobank0.gpio_set_function(UART_RX_PIN, mcal::iobank0::GpioFunction::UART);
  }
}

const UART_TX_PIN: usize = 0;
const UART_RX_PIN: usize = 1;


#[inline(never)]
pub fn c0() -> ! {
  reset_setup();
  gpio_setup();
  multicore_setup();
  unsafe {
    let mut iobank0 = mcal::iobank0::Peripheral::new();
    loop {
        //mcal::sio::SIO.lock().gpio_set(mcal::gpio::GPIO12);
        iobank0.force_high(PICO_DEFAULT_LED_PIN);
        delay(10000000);
        //mcal::gpio::GPIOD.lock().gpio_clear(mcal::gpio::GPIO12);
        iobank0.force_low(PICO_DEFAULT_LED_PIN);
        delay(10000000);
    }
  }
}

#[inline(never)]
pub fn c1() -> ! {
  uart_setup();
  unsafe {
    let mut uart = mcal::uart::Peripheral::new(mcal::uart::Uart::Uart0);
    loop {
        delay(10000000);
        uart.puts("Hello, keso!\n");
    }
  }
}

pub fn main() -> ! {
  delay(100000000);
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
