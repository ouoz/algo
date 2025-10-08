use ac_library::*;
use itertools::*;
use num::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use std::mem::*;
use superslice::*;

fn main() {
    input! {n:usize,mut a:[i64;n]};
    let mut asort = a
        .iter()
        .sorted_by(|a, b| a.cmp(b).reverse())
        .collect::<Vec<_>>();

    let mut alice = 0;
    let mut bob = 0;
    for i in 0..n {
        if i % 2 == 0 {
            alice += asort[i];
        } else {
            bob += asort[i];
        }
    }
    println!("{}", alice - bob);
}
