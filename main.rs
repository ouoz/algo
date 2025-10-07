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
        Y:String
    }
    let mut dict = HashMap::new();
    dict.insert("Ocelot", 0);
    dict.insert("Serval", 1);
    dict.insert("Lynx", 2);
    if dict[&X[..]] >= dict[&Y[..]] {
        println!("Yes");
    } else {
        println!("No");
    }
}
