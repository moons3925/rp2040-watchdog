use rp_pico::hal::watchdog::Watchdog as Wdt;
use rp_pico::hal::pac::WATCHDOG;
pub trait WatchdogPauseOff {    // (1)
    fn pause_off(&self);
}

impl WatchdogPauseOff for Wdt {
    fn pause_off(&self) {   // (2)
        unsafe {(*WATCHDOG::ptr()).ctrl.modify(|_, w| w.pause_dbg0().bit(false).pause_dbg1().bit(false).pause_jtag().bit(false))} // (3)
    }
}
