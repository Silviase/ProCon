use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut f = true;
    let mut g = false;

    for c in s {
        f &= c.is_ascii_lowercase() ^ g;
        g = !g;
    }

    if f {
        println!("Yes")
    }else{
        println!("No")
    }


}
