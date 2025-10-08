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
        a:i64,b:i64,c:i64,s:String,
    };
    println!("{} {}", a + b + c, s);
}
