use ac_library::*;
use itertools::*;
use num::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use std::mem::*;
use superslice::*;

fn main() {
    input! {
        n:usize,a:[i64;n],
    };
    let mut ans = 1_i64 << 60;
    for i in 0..n {
        let mut cur = a[i];
        let mut cnt = 0;
        while cur % 2 == 0 {
            cur /= 2;
            cnt += 1;
        }
        ans = ans.min(cnt);
    }
    println!("{}", ans);
}
