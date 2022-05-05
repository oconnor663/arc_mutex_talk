fn take_a_slice(_: &[i32]) {}

fn take_a_str(_: &str) {}

fn main() {
    let my_vec: Vec<i32> = vec![1, 2, 3];
    take_a_slice(&my_vec);

    let my_string: String = "foo".into();
    take_a_str(&my_string);
}
