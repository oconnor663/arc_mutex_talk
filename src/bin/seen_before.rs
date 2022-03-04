fn take_a_slice(s: &[i32]) {
    dbg!(s.len());
}

fn take_a_str(s: &str) {
    dbg!(s.len());
}

fn main() {
    let my_vec: Vec<i32> = vec![1, 2, 3];
    take_a_slice(&my_vec);

    let my_string: String = "foo".into();
    take_a_str(&my_string);
}
