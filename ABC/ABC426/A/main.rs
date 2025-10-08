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
        X:String,
        Y:String,
    };
    let order = vec!["Ocelot", "Serval", "Lynx"];
    let xi = order.iter().position(|&s| s == X).unwrap();
    let yi = order.iter().position(|&s| s == Y).unwrap();
    if xi >= yi {
        println!("Yes");
    } else {
        println!("No");
    }
}
