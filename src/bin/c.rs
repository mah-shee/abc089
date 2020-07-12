#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        t: [(usize, usize, usize); n],

    }
    // |xi - xi+1| + |yi - yi+1|がt2-t1より大きく、かつ偶奇が一致していればいい
    let mut ans = true;
    for i in 0..(n - 1) {
        let distance = (t[i + 1].1 as isize - t[i].1 as isize).abs()
            + (t[i + 1].2 as isize - t[i].2 as isize).abs();
        if distance as usize <= t[i + 1].0 - t[i].0
            && distance as usize % 2 == (t[i + 1].0 - t[i].0) % 2
        {
            println!("No");
            ans = false;
            break;
        }
    }
    if ans {
        println!("Yes");
    }
}
