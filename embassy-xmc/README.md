# embassy-xmc4700

Embassy-based Rust project foundation for the **Infineon XMC4700 Relax Light Kit**.

## Architecture

- `embassy-executor`: original Embassy async executor, Cortex-M thread mode.
- `embassy-time`: Embassy timer API.
- `xmc4700`: generated PAC for register-level access.
- `cortex-m` / `cortex-m-rt`: Cortex-M runtime and core support.

Target: `thumbv7em-none-eabihf` (Cortex-M4F).

## Important status

This is an initial **HAL project scaffold**, not yet a ready-to-flash board-support package.
It intentionally does not guess XMC4700 register addresses, clock frequencies, pin mux
settings, or LED polarity. In particular:

1. GPIO register backend is not implemented yet.
2. XMC clock-tree and peripheral clock setup are not implemented yet.
3. An `embassy-time-driver` backed by an XMC CCU4/CCU8 timer (or a verified SysTick
   arrangement) is not registered yet. `Timer::after(...)` must not be used until then.
4. Startup/linker memory layout and the exact Relax Kit board revision still need
   verification against the board documentation.

The executor dependency is the real upstream Embassy executor. The project does not
contain a replacement executor.

## Build environment

Install the Rust target:

```sh
rustup target add thumbv7em-none-eabihf
rustup component add rust-src
```

Build the workspace:

```sh
cargo check --workspace
```

The target linker script and memory layout may need to be added for the selected
board startup/runtime arrangement before producing a flashable ELF.

## Planned implementation order

1. Confirm Relax Kit revision, oscillator, and linker memory map.
2. Implement `SystemInit`/clock tree and flash wait states.
3. Implement safe GPIO pin ownership and LED 5.8 output.
4. Implement a monotonic 64-bit Embassy time driver using a CCU timer.
5. Add USIC SPI, I2C and UART drivers.
6. Add board examples: LED heartbeat, I2C scan, SPI TFT (ILI9341/ST7796).

## Why a CCU timer for Embassy time?

`embassy-time` requires a monotonic timestamp and a scheduled wake-up callback.
A free-running hardware timer plus compare interrupt is a better fit than a
busy-loop delay. The driver will extend the hardware counter to 64 bits and
maintain the Embassy timer queue.
