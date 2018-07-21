#![no_std]
#![no_main]

extern crate panic_semihosting;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;

#[macro_use(entry, exception)]
extern crate microbit;

use core::fmt::Write;
use rt::ExceptionFrame;
use sh::hio;

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

entry!(main);
fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    stdout.write_str("semihosting test\n\r").unwrap();

    if let Some(p) = microbit::Peripherals::take() {
        let mut gpio = p.GPIO.split();
        // Configure RX and TX pins accordingly
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();
        let _ = write!(tx, "\n\rserial test\n\r");
    }

    panic!("test-panic");
}
