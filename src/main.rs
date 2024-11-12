#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin};
use panic_halt as _;
use rp2040_hal::{pac, Clock, Sio};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = rp2040_hal::Watchdog::new(pac.WATCHDOG);

    let clocks = rp2040_hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = Sio::new(pac.SIO);

    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // LED_PIN = GPIO16 & GPIO17
    let mut led_pin1 = pins.gpio16.into_push_pull_output();
    let mut led_pin2 = pins.gpio17.into_push_pull_output();
    // SWITCH_PIN = GPIO3
    let mut switch_pin = pins.gpio3.into_pull_up_input();

    loop {
        let switch_state = switch_pin.is_low().unwrap();

        if switch_state {
            led_pin1.set_low().unwrap();
            led_pin2.set_low().unwrap();
        } else {
            while !switch_pin.is_low().unwrap() {
                led_pin1.set_high().unwrap();
                led_pin2.set_low().unwrap();
                delay.delay_ms(200);
                led_pin2.set_high().unwrap();
                led_pin1.set_low().unwrap();
                delay.delay_ms(200);
            }
        }
        delay.delay_ms(100);
    }
}
