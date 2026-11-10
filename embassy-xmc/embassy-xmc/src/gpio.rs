//! GPIO API foundation.
//!
//! Pin routing is intentionally not guessed. The XMC4700 has per-pin
//! alternate-function selections and the Relax Kit board exposes several
//! possible LED/display routes. The concrete register backend is the next
//! hardware implementation step.

use core::ptr::{read_volatile, write_volatile};

const PORT0_BASE: usize = 0x4802_8000;
const PORT1_BASE: usize = 0x4802_8100;
const PORT2_BASE: usize = 0x4802_8200;
const PORT3_BASE: usize = 0x4802_8300;
const PORT4_BASE: usize = 0x4802_8400;
const PORT5_BASE: usize = 0x4802_8500;
const PORT6_BASE: usize = 0x4802_8600;
const PORT7_BASE: usize = 0x4802_8700;
const PORT8_BASE: usize = 0x4802_8800;
const PORT9_BASE: usize = 0x4802_8900;
// Port 10-13 existieren nicht
const PORT14_BASE: usize = 0x4802_8E00;
const PORT15_BASE: usize = 0x4802_8F00;

/// Offsets der Port-Register (für alle Ports P0..P9 identisch).
const OMR: usize = 0x04; // Output Modification Register
const IOCR0: usize = 0x10; // Input/Output Control Register für Pins 0..3 (IOCR4 = 0x14, ...)

/// PCx-Feld in IOCRn: General-Purpose-Ausgang, Push-Pull.
const IOCR_PC_OUTPUT_PUSH_PULL: u32 = 0b1_0000;
/// Höchster Port mit Ausgangsregistern. P14/P15 sind reine Analog-/Digital-Eingänge.
const MAX_OUTPUT_PORT: u8 = 9;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Level {
    Low,
    High,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Input,
    Output,
}

/// Describes a GPIO pin after the board-specific pin mux has been configured.
pub struct Pin {
    port: u8,
    pin: u8,
}

impl Pin {
    /// Construct a pin descriptor. This is an internal low-level escape hatch;
    /// application code should use board pin constants when those are added.
    pub const unsafe fn new_unchecked(port: u8, pin: u8) -> Self {
        assert!(pin < 16, "Invalid pin");
        Self { port, pin }
    }

    pub const fn port(&self) -> u8 {
        self.port
    }

    pub const fn number(&self) -> u8 {
        self.pin
    }

    /// Basisadresse des Ports als Zahl (Byte-Adresse).
    const fn port_base(&self) -> usize {
        match self.port {
            0 => PORT0_BASE,
            1 => PORT1_BASE,
            2 => PORT2_BASE,
            3 => PORT3_BASE,
            4 => PORT4_BASE,
            5 => PORT5_BASE,
            6 => PORT6_BASE,
            7 => PORT7_BASE,
            8 => PORT8_BASE,
            9 => PORT9_BASE,
            14 => PORT14_BASE,
            15 => PORT15_BASE,
            _ => panic!("Invalid port"),
        }
    }

    pub const fn port_adr(&self) -> *mut u32 {
        self.port_base() as *mut u32
    }

    /// Adresse eines Port-Registers. Der Offset ist in **Bytes**.
    ///
    /// Achtung: `*mut u32::add(n)` rechnet in Elementen (n * 4 Bytes). Daher wird
    /// hier auf `usize` addiert und erst danach zum Pointer gecastet.
    const fn reg(&self, offset: usize) -> *mut u32 {
        (self.port_base() + offset) as *mut u32
    }

    /// Adresse des Output Modification Register (OMR) dieses Ports.
    pub const fn omr_adr(&self) -> *mut u32 {
        self.reg(OMR)
    }

    /// Schreibt das PCx-Feld dieses Pins im passenden IOCRn-Register.
    fn set_pc(&self, pc: u32) {
        // 4 Pins pro IOCR-Register, je 8 Bit; PCx liegt in den Bits [7:3] des Bytes.
        let reg = self.reg(IOCR0 + 4 * (self.pin as usize / 4));
        let shift = 8 * (self.pin as u32 % 4) + 3;
        // IOCR deckt vier Pins ab: Read-Modify-Write gegen Unterbrechungen absichern.
        critical_section::with(|_| unsafe {
            let v = read_volatile(reg);
            write_volatile(reg, (v & !(0x1F << shift)) | (pc << shift));
        });
    }
}

/// Output pin handle (Push-Pull).
pub struct Output {
    pin: Pin,
}

impl Output {
    pub fn new(pin: Pin, initial: Level) -> Self {
        assert!(
            pin.port <= MAX_OUTPUT_PORT,
            "Port hat keine Ausgangsregister"
        );
        let mut out = Self { pin };
        // Erst den Pegel setzen, dann den Ausgang aktivieren: kein Glitch beim Start.
        out.set_level(initial);
        out.pin.set_pc(IOCR_PC_OUTPUT_PUSH_PULL);
        out
    }

    pub fn set_level(&mut self, level: Level) {
        let pin = self.pin.pin as u32;
        // OMR: Bits 0..15 = PSn (setzen), Bits 16..31 = PRn (löschen).
        // Beides liegt im selben Register; ein einzelner Schreibzugriff genügt.
        let value = match level {
            Level::High => 1u32 << pin,
            Level::Low => 1u32 << (pin + 16),
        };
        unsafe { write_volatile(self.pin.omr_adr(), value) }
    }

    pub fn toggle(&mut self) {
        let pin = self.pin.pin as u32;
        // PSn und PRn gleichzeitig gesetzt => Toggle, atomar in Hardware.
        unsafe { write_volatile(self.pin.omr_adr(), (1u32 << pin) | (1u32 << (pin + 16))) }
    }

    pub const fn pin(&self) -> &Pin {
        &self.pin
    }
}