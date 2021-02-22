use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        x: usize,
    }

    println!("{}", 100 - (x - x/100*100))
}
