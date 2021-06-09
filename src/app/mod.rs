pub mod mcal;

pub const PICO_DEFAULT_LED_PIN: usize = 25;

 #[inline(never)]
fn delay(ticks: usize) {
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

pub fn main() -> ! {
  //delay(1000000000);
  reset_setup();
  gpio_setup();
  uart_setup();
  unsafe {
    let mut iobank0 = mcal::iobank0::Peripheral::new();
    let mut uart = mcal::uart::Peripheral::new(mcal::uart::Uart::Uart0);
    loop {
        //mcal::sio::SIO.lock().gpio_set(mcal::gpio::GPIO12);
        iobank0.force_high(PICO_DEFAULT_LED_PIN);
        delay(10000000);
        //mcal::gpio::GPIOD.lock().gpio_clear(mcal::gpio::GPIO12);
        iobank0.force_low(PICO_DEFAULT_LED_PIN);
        delay(10000000);

        uart.puts("Hello, UART!\n");
    }
  }
}
