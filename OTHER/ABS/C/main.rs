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
        sc:Chars,
    };
    let mut ans = 0;
    for i in 0..sc.len() {
        if sc[i] == '1' {
            ans += 1;
        }
    }
    println!("{}", ans);
}
