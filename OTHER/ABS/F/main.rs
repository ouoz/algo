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
        n:usize,a:i64,b:i64,
    };
    let mut ans = 0;
    for i in 1..=n {
        let mut cur = i as i64;
        let fx = f(cur);
        if fx >= a && fx <= b {
            ans += cur;
        }
    }
    println!("{}", ans);
}

fn f(mut x: i64) -> i64 {
    let mut ret = 0;
    while x > 0 {
        ret += x % 10;
        x /= 10;
    }
    ret
}
