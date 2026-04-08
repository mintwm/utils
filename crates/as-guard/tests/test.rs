use as_guard::AsGuard;

#[test]
#[should_panic = "assertion failed: <i8>::try_from(self).is_ok()"]
fn u64_as_i8_panic() {
    let max = u64::MAX;
    let _: i8 = max.safe_as();
}

#[test]
fn u64_as_i8() {
    let value = 0u64;
    let _: i8 = value.safe_as();
}

#[test]
fn u8_as_isize() {
    let max = u8::MAX;
    let _: isize = max.safe_as();
}
