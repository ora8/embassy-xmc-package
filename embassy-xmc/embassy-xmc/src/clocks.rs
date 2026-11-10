//! XMC4700 clock-tree configuration.
//!
//! Do not assume a clock frequency until the board oscillator and PLL setup
//! are known. The Relax Kit's startup configuration must be verified against
//! the exact kit revision and schematic.

#[derive(Clone, Copy, Debug)]
pub struct Clocks {
    pub f_cpu_hz: u32,
    pub f_peripheral_hz: u32,
}

/// Conservative descriptor only; it does not configure hardware.
pub const UNCONFIGURED: Clocks = Clocks {
    f_cpu_hz: 0,
    f_peripheral_hz: 0,
};
