//! Embassy-Zeittreiber für den XMC4700 auf Basis des Cortex-M-SysTick.
//!
//! SysTick löst alle 1 ms einen Interrupt aus (Tick-Rate 1 kHz, passend zum Feature
//! `tick-hz-1_000` von embassy-time). Der Handler zählt die Zeit hoch und weckt alle Tasks,
//! deren Timer abgelaufen sind. Auflösung und maximale Weckverzögerung: 1 ms.

use core::cell::{Cell, RefCell};
use core::task::Waker;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;
use cortex_m_rt::exception;
use critical_section::Mutex;
use embassy_time_driver::{time_driver_impl, Driver};
use embassy_time_queue_utils::Queue;

pub const TICK_HZ: u32 = 1_000;

struct XmcTimeDriver {
    ticks: Mutex<Cell<u64>>,
    queue: Mutex<RefCell<Queue>>,
}

time_driver_impl!(static DRIVER: XmcTimeDriver = XmcTimeDriver {
    ticks: Mutex::new(Cell::new(0)),
    queue: Mutex::new(RefCell::new(Queue::new())),
});

impl Driver for XmcTimeDriver {
    fn now(&self) -> u64 {
        critical_section::with(|cs| self.ticks.borrow(cs).get())
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        critical_section::with(|cs| {
            // Liegt "at" schon in der Vergangenheit, weckt der nächste SysTick-Tick den Task
            self.queue.borrow(cs).borrow_mut().schedule_wake(at, waker);
        });
    }
}

impl XmcTimeDriver {
    fn on_tick(&self) {
        critical_section::with(|cs| {
            let now = self.ticks.borrow(cs).get() + 1;
            self.ticks.borrow(cs).set(now);
            // weckt alle abgelaufenen Einträge
            self.queue.borrow(cs).borrow_mut().next_expiration(now);
        });
    }
}

#[exception]
fn SysTick() {
    DRIVER.on_tick();
}

/// SysTick starten. Muss vor der ersten Verwendung von `embassy_time` aufgerufen werden.
pub fn init(mut syst: SYST, core_hz: u32) {
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(core_hz / TICK_HZ - 1);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
}
