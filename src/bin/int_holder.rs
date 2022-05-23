use std::ops::{Deref, DerefMut};

struct IntHolder {
    the_int: i32,
}

impl Deref for IntHolder {
    type Target = i32;

    fn deref(&self) -> &i32 {
        println!("deref!");
        &self.the_int
    }
}

impl DerefMut for IntHolder {
    fn deref_mut(&mut self) -> &mut i32 {
        println!("deref_mut!");
        &mut self.the_int
    }
}

fn main() {
    let mut holder = IntHolder { the_int: 42 };
    assert_eq!(42, *holder);
    *holder = 43;
    assert_eq!(43, *holder);
}
