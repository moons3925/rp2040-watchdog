#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
};
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rp_pico as bsp;

use embedded_hal::watchdog::{Watchdog, WatchdogEnable}; // (1) feed() , start()

use fugit::ExtU32; // (2) millis()

//use rp2040_lib::watchdog::WatchdogPauseOff; // (3) 追加するトレイトを使えるようにする

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = bsp::hal::watchdog::Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut led_pin = pins.led.into_push_pull_output();

    led_pin.set_high().unwrap();
    delay.delay_ms(3000);

    watchdog.start(3000.millis()); // (4)

    //    watchdog.pause_off(); // (5)

    for _ in 0..5 {
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        watchdog.feed(); // (6)
    }

    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(100);
        led_pin.set_low().unwrap();
        delay.delay_ms(100);
    }
}
