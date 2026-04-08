use idmint::StackMint;

#[test]
fn issue_and_recycle_last() {
    let mut mint = StackMint::new(0);
    assert_eq!(mint.last(), 0);
    assert_eq!(mint.issue(), Some(0));
    assert_eq!(mint.issue(), Some(1));
    assert_eq!(mint.last(), 1);
    mint.recycle(1);
    mint.recycle(0);
    assert_eq!(mint.last(), 0);
}

#[test]
fn issue_and_recycle_middle() {
    let mut mint = StackMint::new(1);
    assert_eq!(mint.issue(), Some(1));
    assert_eq!(mint.issue(), Some(2));
    assert_eq!(mint.issue(), Some(3));

    assert_eq!(mint.used(), 4); // 3 + Reserved
    mint.recycle(2);

    assert_eq!(mint.used(), 3); // [0, 1, _, 3]
    assert!(mint.is_value_in_use(0));
    assert!(mint.is_value_in_use(1));
    assert!(!mint.is_value_in_use(2)); // Recycled
    assert!(mint.is_value_in_use(3));

    assert_eq!(mint.issue(), Some(2));
}

#[test]
fn reserve() {
    // Reserved: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] (10 elements)
    let mint = StackMint::new(10);
    assert_eq!(mint.last(), 9);
}

#[test]
fn overflow() {
    let mut mint = StackMint::new(u8::MAX);
    assert_eq!(mint.issue(), None);
}

#[test]
fn recycle_recycled_last() {
    let mut mint = StackMint::new(1);
    let _ = mint.issue();
    let v1 = mint.issue().unwrap();

    mint.recycle(v1);

    let mint_clone = mint.clone();

    mint.recycle(v1);

    assert_eq!(mint, mint_clone);
}

#[test]
fn recycle_recycled_middle() {
    let mut mint = StackMint::new(1);
    let _ = mint.issue();
    let v1 = mint.issue().unwrap();
    let _ = mint.issue();

    mint.recycle(v1);

    let mint_clone = mint.clone();

    mint.recycle(v1);

    assert_eq!(mint, mint_clone);
}
