#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![no_std]

#[cfg(feature = "heap")]
mod heap;

#[cfg(feature = "stack")]
mod stack;

#[cfg(feature = "heap")]
pub use heap::*;

#[cfg(feature = "stack")]
pub use stack::*;
