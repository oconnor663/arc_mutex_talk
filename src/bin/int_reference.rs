fn main() {
    let mut foo = 42;
    let refmutfoo = &mut foo;
    assert_eq!(42, *refmutfoo);
    *refmutfoo = 43;
    assert_eq!(43, *refmutfoo);
}
