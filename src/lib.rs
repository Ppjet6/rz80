//! **rz80** is a Z80 chip family emulation library written in Rust which can be used as basis for
//! writing a full computer emulator.
//!

/// generic integer type for 8- and 16-bit values
pub type RegT = i32;

mod registers;
mod memory;
mod bus;
mod cpu;
mod pio;
mod ctc;
mod daisychain;

pub use registers::{Registers, CF, NF, VF, PF, XF, HF, YF, ZF, SF};
pub use memory::Memory;
pub use cpu::CPU;
pub use bus::Bus;
pub use pio::{PIO, PIO_A, PIO_B};
pub use ctc::{CTC, CTC_0, CTC_1, CTC_2, CTC_3};
pub use daisychain::Daisychain;
