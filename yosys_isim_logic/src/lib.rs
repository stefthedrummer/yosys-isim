#![feature(portable_simd)]

#[cfg(test)]
mod tests;

/*internal*/
mod simd;

pub mod logic;
pub use logic::*;

pub mod logic_slice;
pub use logic_slice::*;

// pub mod logic_vec;
// pub use logic_vec::*;

pub mod logic_array64;
pub use logic_array64::*;

pub mod ops;
pub use ops::*;

fn __div_ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}
