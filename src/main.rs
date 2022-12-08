#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");

    let mut name: String = String::new();

    io::stdin().read_line(&mut name)
        .expect("Name is required.");

    println!("Hello, {}!", name.trim_end());

    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.1415;

    let age = "22";
    let mut age: u32 = age.trim().parse().expect("Age is required.");

    print!("{}, you have {} years", name.trim_end(), age);
}
