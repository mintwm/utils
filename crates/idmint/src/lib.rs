#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![no_std]

mod heap;
mod stack;

pub use heap::*;
pub use stack::*;
