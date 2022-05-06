fn main() {
    let foo: i32 = 42;
    let foo_ref: &i32 = &foo;

    // warning: unused unary operation that must be used
    *foo_ref;

    let bar: i32 = *foo_ref;
    assert_eq!(bar, 42);

    let mut foo: i32 = 42;
    let foo_ref_mut: &mut i32 = &mut foo;
    *foo_ref_mut = 99;
    assert_eq!(foo, 99);
}
