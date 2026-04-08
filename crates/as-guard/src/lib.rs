#![no_std]

macro_rules! define_as_guard_runtime {
    ($from:ty, $to:ty) => {
        define_as_guard_runtime!($from, $to, s => s);
    };
    ($from:ty, $to:ty, $ident:ident => $cast:expr) => {
        impl $crate::AsGuardTo<$to> for $from {
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::inline_always)]
            #[inline(always)]
            #[doc = "Performs type casting with additional safety checks."]
            #[doc = ""]
            #[doc = "This function should be used when the value is expected to fit within the"]
            #[doc = "target type's range. It helps track where an overflow occurred in case"]
            #[doc = "of logic errors during development."]
            fn convert(self) -> $to {
                let $ident = self;
                cfg_if::cfg_if! {
                    if #[cfg(feature = "debug")] {
                        debug_assert!(<$to>::try_from(self).is_ok());
                    } else if #[cfg(feature = "release")] {
                        assert!(<$to>::try_from(self).is_ok());
                    } else {
                        panic!("Select 'debug' or 'release' feature");
                    }
                }
                $cast as $to
            }
        }
    };
}

pub trait AsGuardTo<T> {
    fn convert(self) -> T;
}

pub trait AsGuard {
    fn safe_as<T>(self) -> T
    where
        Self: AsGuardTo<T>;
}

impl<S> AsGuard for S {
    #[inline]
    #[doc = "Performs type casting with additional safety checks."]
    #[doc = ""]
    #[doc = "This function should be used when the value is expected to fit within the"]
    #[doc = "target type's range. It helps track where an overflow occurred in case"]
    #[doc = "of logic errors during development."]
    fn safe_as<T>(self) -> T
    where
        Self: AsGuardTo<T>,
    {
        AsGuardTo::convert(self)
    }
}

define_as_guard_runtime!(usize, u8);
define_as_guard_runtime!(usize, u16);
define_as_guard_runtime!(usize, u32);
define_as_guard_runtime!(usize, u64);
define_as_guard_runtime!(usize, u128);

define_as_guard_runtime!(isize, i8);
define_as_guard_runtime!(isize, i16);
define_as_guard_runtime!(isize, i32);
define_as_guard_runtime!(isize, i64);
define_as_guard_runtime!(isize, i128);

define_as_guard_runtime!(i8, u8, s => s.cast_unsigned());
define_as_guard_runtime!(i8, u16, s => s.cast_unsigned());
define_as_guard_runtime!(i8, u32, s => s.cast_unsigned());
define_as_guard_runtime!(i8, u64, s => s.cast_unsigned());
define_as_guard_runtime!(i8, u128, s => s.cast_unsigned());
define_as_guard_runtime!(i8, usize, s => s.cast_unsigned());

define_as_guard_runtime!(i16, u8, s => s.cast_unsigned());
define_as_guard_runtime!(i16, u16, s => s.cast_unsigned());
define_as_guard_runtime!(i16, u32, s => s.cast_unsigned());
define_as_guard_runtime!(i16, u64, s => s.cast_unsigned());
define_as_guard_runtime!(i16, u128, s => s.cast_unsigned());
define_as_guard_runtime!(i16, usize, s => s.cast_unsigned());

define_as_guard_runtime!(i32, u8, s => s.cast_unsigned());
define_as_guard_runtime!(i32, u16, s => s.cast_unsigned());
define_as_guard_runtime!(i32, u32, s => s.cast_unsigned());
define_as_guard_runtime!(i32, u64, s => s.cast_unsigned());
define_as_guard_runtime!(i32, u128, s => s.cast_unsigned());
define_as_guard_runtime!(i32, usize, s => s.cast_unsigned());

define_as_guard_runtime!(i64, u8, s => s.cast_unsigned());
define_as_guard_runtime!(i64, u16, s => s.cast_unsigned());
define_as_guard_runtime!(i64, u32, s => s.cast_unsigned());
define_as_guard_runtime!(i64, u64, s => s.cast_unsigned());
define_as_guard_runtime!(i64, u128, s => s.cast_unsigned());
define_as_guard_runtime!(i64, usize, s => s.cast_unsigned());

define_as_guard_runtime!(i128, u8, s => s.cast_unsigned());
define_as_guard_runtime!(i128, u16, s => s.cast_unsigned());
define_as_guard_runtime!(i128, u32, s => s.cast_unsigned());
define_as_guard_runtime!(i128, u64, s => s.cast_unsigned());
define_as_guard_runtime!(i128, u128, s => s.cast_unsigned());
define_as_guard_runtime!(i128, usize, s => s.cast_unsigned());

define_as_guard_runtime!(isize, u8, s => s.cast_unsigned());
define_as_guard_runtime!(isize, u16, s => s.cast_unsigned());
define_as_guard_runtime!(isize, u32, s => s.cast_unsigned());
define_as_guard_runtime!(isize, u64, s => s.cast_unsigned());
define_as_guard_runtime!(isize, u128, s => s.cast_unsigned());
define_as_guard_runtime!(isize, usize, s => s.cast_unsigned());

define_as_guard_runtime!(u8, i8, s => s.cast_signed());
define_as_guard_runtime!(u8, i16, s => s.cast_signed());
define_as_guard_runtime!(u8, i32, s => s.cast_signed());
define_as_guard_runtime!(u8, i64, s => s.cast_signed());
define_as_guard_runtime!(u8, i128, s => s.cast_signed());
define_as_guard_runtime!(u8, isize, s => s.cast_signed());

define_as_guard_runtime!(u16, i8, s => s.cast_signed());
define_as_guard_runtime!(u16, i16, s => s.cast_signed());
define_as_guard_runtime!(u16, i32, s => s.cast_signed());
define_as_guard_runtime!(u16, i64, s => s.cast_signed());
define_as_guard_runtime!(u16, i128, s => s.cast_signed());
define_as_guard_runtime!(u16, isize, s => s.cast_signed());

define_as_guard_runtime!(u32, i8, s => s.cast_signed());
define_as_guard_runtime!(u32, i16, s => s.cast_signed());
define_as_guard_runtime!(u32, i32, s => s.cast_signed());
define_as_guard_runtime!(u32, i64, s => s.cast_signed());
define_as_guard_runtime!(u32, i128, s => s.cast_signed());
define_as_guard_runtime!(u32, isize, s => s.cast_signed());

define_as_guard_runtime!(u64, i8, s => s.cast_signed());
define_as_guard_runtime!(u64, i16, s => s.cast_signed());
define_as_guard_runtime!(u64, i32, s => s.cast_signed());
define_as_guard_runtime!(u64, i64, s => s.cast_signed());
define_as_guard_runtime!(u64, i128, s => s.cast_signed());
define_as_guard_runtime!(u64, isize, s => s.cast_signed());

define_as_guard_runtime!(u128, i8, s => s.cast_signed());
define_as_guard_runtime!(u128, i16, s => s.cast_signed());
define_as_guard_runtime!(u128, i32, s => s.cast_signed());
define_as_guard_runtime!(u128, i64, s => s.cast_signed());
define_as_guard_runtime!(u128, i128, s => s.cast_signed());
define_as_guard_runtime!(u128, isize, s => s.cast_signed());

define_as_guard_runtime!(usize, i8, s => s.cast_signed());
define_as_guard_runtime!(usize, i16, s => s.cast_signed());
define_as_guard_runtime!(usize, i32, s => s.cast_signed());
define_as_guard_runtime!(usize, i64, s => s.cast_signed());
define_as_guard_runtime!(usize, i128, s => s.cast_signed());
define_as_guard_runtime!(usize, isize, s => s.cast_signed());

define_as_guard_runtime!(f32, f64, s => f64::from(s));
