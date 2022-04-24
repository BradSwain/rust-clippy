#![warn(clippy::empty_vec_call)]

fn side_effect() -> usize {
    println!("side effect !");
    0
}

fn main() {
    let _a = [side_effect(); 0];
    let _v = vec![side_effect(); 0];

    // Should not emit warning because there are no side effects
    let _ok_a = [0; 0];
    let _ok_v = vec![0; 0];
}
