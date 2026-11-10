#![no_std]
#![no_main]

use panic_halt as _;
use core::ptr::{read_volatile};
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_xmc::gpio::{Level, Output, Pin};
use embassy_time::{Duration, Ticker};

use embassy_xmc::time_drivers as time_driver;

const SCU_CLK_SYSCLKCR: usize = 0x5000_460C;
const SCU_CLK_CPUCLKCR: usize = 0x5000_4610;

/// Takt, den der Kern annimmt, wenn die PLL als Systemtakt gewählt ist (hier nicht konfiguriert)
const FALLBACK_CORE_HZ: u32 = 24_000_000;

/// Signal von led1_task an counter_task (trägt den Zählerstand)
static BLINK_EVENT: Signal<CriticalSectionRawMutex, u32> = Signal::new();

// Kerntakt aus den SCU-Registern lesen. Nach dem Reset läuft der Kern mit dem
/// 24-MHz-Backup-Takt (fOFI) geteilt durch (SYSDIV+1) und (CPUDIV+1).
fn core_clock_hz() -> u32 {
    let sysclkcr = unsafe { read_volatile(SCU_CLK_SYSCLKCR as *const u32) };
    let cpuclkcr = unsafe { read_volatile(SCU_CLK_CPUCLKCR as *const u32) };
    let syssel_pll = sysclkcr & (1 << 16) != 0;
    if syssel_pll {
        FALLBACK_CORE_HZ
    } else {
        let sysdiv = (sysclkcr & 0xFF) + 1;
        let cpudiv = (cpuclkcr & 0x1) + 1;
        24_000_000 / sysdiv / cpudiv
    }
}

#[embassy_executor::task]
async fn led1_task() {
    // Ticker statt Timer::after: kein Drift, die Periode bleibt exakt
    let mut ticker = Ticker::every(Duration::from_millis(500));
    let mut n: u32 = 0;
    let led_pin = unsafe { Pin::new_unchecked(5, 8) }; // Beispiel für LED Pin auf Port 5, Pin 8
    let mut led = Output::new(led_pin, Level::Low);
    loop {
        led.toggle();
        n = n.wrapping_add(1);
        BLINK_EVENT.signal(n);
        ticker.next().await;
    }
}

/// Minimal executor bring-up.
///
/// This proves the upstream Embassy executor is selected and entered.
/// Replace `pending()` with spawned application tasks once GPIO and the
/// XMC hardware time driver are implemented.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let core = cortex_m::Peripherals::take().unwrap();
    time_driver::init(core.SYST, core_clock_hz());
    spawner.spawn(led1_task().unwrap());
}
