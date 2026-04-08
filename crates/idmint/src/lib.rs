#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![no_std]

use as_guard::AsGuard;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "partial_ord", derive(PartialOrd, Ord))]
#[cfg_attr(feature = "partial_eq", derive(PartialEq, Eq))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub struct StackMint {
    next: u8,
    returned: u8,
    bytes: [u8; 256],
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl Default for StackMint {
    fn default() -> Self {
        Self::new(0)
    }
}

impl StackMint {
    #[must_use]
    pub const fn new(reserved: u8) -> Self {
        Self {
            next: reserved,
            returned: 0,
            bytes: [0; 256],
        }
    }

    #[must_use]
    pub fn used(&self) -> u8 {
        self.next - self.returned
    }

    /// Returns the last used value (in the entire range)
    #[must_use]
    pub fn last(&self) -> u8 {
        self.next.saturating_sub(1)
    }

    #[must_use]
    pub fn is_value_in_use(&self, value: u8) -> bool {
        value < self.next && !self.get_bit(value as usize)
    }

    #[must_use]
    pub fn issue(&mut self) -> Option<u8> {
        if self.returned != 0 {
            // SAFETY: if self.returned is non-zero, then at least one byte is non-zero.
            let (index, range) = unsafe {
                self.bytes
                    .iter()
                    .copied()
                    .enumerate()
                    .find(|&(_, value)| value != 0)
                    .unwrap_unchecked()
            };

            for i in 0..8u8 {
                let bit = (range >> i) & 0b_00000001;
                if bit == 1 {
                    self.set_bit(index, false);
                    return Some(index.safe_as::<u8>() + bit + 1);
                }
            }
        }

        if self.next != u8::MAX {
            let val = self.next;
            self.next += 1;
            return Some(val);
        }

        None
    }

    pub fn recycle(&mut self, value: u8) {
        if value == self.next || self.get_bit(value as usize) {
            return;
        }

        if value == self.next - 1 {
            self.next -= 1;
        } else {
            self.set_bit(value as usize, true);
            self.returned += 1;
        }
    }

    fn get_bit(&self, index: usize) -> bool {
        let byte_index = index / 8;
        (self.bytes[byte_index] >> index) & 1 == 1
    }

    fn set_bit(&mut self, index: usize, value: bool) {
        let byte_index = index / 8;
        let mask = 1 << index;
        if value {
            self.bytes[byte_index] |= mask;
        } else {
            self.bytes[byte_index] &= !mask;
        }
    }
}
