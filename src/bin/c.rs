#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let (mut m, mut a, mut r, mut c, mut h) = (0, 0, 0, 0, 0);
    for i in 0..n {
        if s[i][0] == 'M' {
            m += 1
        }
        if s[i][0] == 'A' {
            a += 1
        }
        if s[i][0] == 'R' {
            r += 1;
        }
        if s[i][0] == 'C' {
            c += 1;
        }
        if s[i][0] == 'H' {
            h += 1;
        }
    }
    let d = vec![m, a, r, c, h];
    let p = [0, 0, 0, 0, 0, 0, 1, 1, 1, 2];
    let q = [1, 1, 1, 2, 2, 3, 2, 2, 3, 3];
    let t = [2, 3, 4, 3, 4, 4, 3, 4, 4, 4];
    let mut ans: u128 = 0;
    for i in 0..10 {
        ans += d[p[i]] * d[q[i]] * d[t[i]];
    }
    println!("{}", ans);
}
