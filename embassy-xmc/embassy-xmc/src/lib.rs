#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

//! HAL foundation for the Infineon XMC4700.
//!
//! This crate deliberately keeps the hardware-specific layer separate from
//! Embassy's executor and time APIs. Clock-tree and peripheral pin routing
//! must be configured for the exact Relax Kit board revision.

pub mod gpio;
pub mod interrupt;
pub mod clocks;
pub mod time_drivers;

/// Board/peripheral ownership token.
///
/// The final `Peripherals::take()` implementation will enforce singleton
/// ownership for each peripheral. This initial scaffold does not yet claim
/// ownership of PAC peripherals.
pub struct Peripherals {
    _private: (),
}

impl Peripherals {
    /// Initialize the XMC4700 HAL.
    ///
    /// TODO: implement clock tree, flash wait states, peripheral clock gates,
    /// and board-specific pin mux setup after confirming the board revision.
    pub fn take() -> Self {
        Self { _private: () }
    }
}
