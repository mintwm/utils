#![cfg(feature = "heap")]

extern crate alloc;

use core::hash::Hash;
use core::ops::{AddAssign, SubAssign};

use indexmap::IndexSet;
use num_traits::{PrimInt, Unsigned};

pub trait Coin: PrimInt + AddAssign + SubAssign + Clone + PartialEq + Unsigned + Hash {}

impl Coin for u8 {}
impl Coin for u16 {}
impl Coin for u32 {}
impl Coin for u64 {}
impl Coin for u128 {}
impl Coin for usize {}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "partial_eq", derive(PartialEq, Eq))]
pub struct HeapMint<C: Coin> {
    next: C,
    returned: IndexSet<C>,
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl<C: Coin> Default for HeapMint<C> {
    fn default() -> Self {
        Self::new(C::zero())
    }
}

impl<C: Coin> HeapMint<C> {
    #[must_use]
    pub fn new(reserved: C) -> Self {
        Self {
            next: reserved,
            returned: IndexSet::new(),
        }
    }

    #[must_use]
    pub fn used(&self) -> C {
        self.next - unsafe { C::from(self.returned.len()).unwrap_unchecked() }
    }

    /// Returns the last used value (in the entire range)
    #[must_use]
    pub fn last(&self) -> C {
        self.next.saturating_sub(C::one())
    }

    #[must_use]
    pub fn is_value_in_use(&self, value: C) -> bool {
        value < self.next && !self.is_returned(value)
    }

    #[must_use]
    pub fn issue(&mut self) -> Option<C> {
        if !self.returned.is_empty() {
            // SAFETY: if self.returned is non-zero, then at least one byte is non-zero.
            let first = unsafe { *self.returned.first().unwrap_unchecked() };
            self.returned.shift_remove(&first);
            return Some(first);
        }

        if self.next != C::max_value() {
            let val = self.next;
            self.next += C::one();
            return Some(val);
        }

        None
    }

    pub fn recycle(&mut self, value: C) {
        if value == self.next || self.is_returned(value) {
            return;
        }

        if value == self.next - C::one() {
            self.next -= C::one();
        } else {
            self.add_to_returned(value);
        }
    }

    fn is_returned(&self, value: C) -> bool {
        self.returned.contains(&value)
    }

    fn add_to_returned(&mut self, value: C) {
        self.returned.insert_sorted(value);
    }
}
