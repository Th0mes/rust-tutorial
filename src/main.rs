#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {

}

fn what_is_your_name() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didn't receive input");
    println!("Hello, {}! {}", name.trim_end(), greeting);
}

fn mutable_statement() {
    let mut number = 6;
    conditional_checking(number);
    number = 3;
    conditional_checking(number);
    fn conditional_checking(cond: i32) {
        if cond < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    };
}

fn same_name_another_type() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assined to a number");
    age = age + 1;
    println!("I'm {age} and I want ${ONE_MIL}");
}

fn data_types() {
    // print all data types max size
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    let is_true: bool = true;    // boolean
    let my_grade: char = 'A';    // character

    let num_1: f32 = 1.11111111111111;
    println!("f32: {}", num_1 + 0.11111111111111);  // less precision
    let num_2: f64 = 1.11111111111111;
    println!("f64: {}", num_2 + 0.11111111111111);  // more precision
}

fn math() {
    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("{num_3} + {num_4} = {}", num_3 + num_4);
    println!("{num_3} - {num_4} = {}", num_3 - num_4);
    println!("{num_3} * {num_4} = {}", num_3 * num_4);
    println!("{num_3} / {num_4} = {}", num_3 / num_4);
    println!("{num_3} % {num_4} = {}", num_3 % num_4);
}

fn counter_loop() {
    let mut counter = 0;

    let result = loop {
        println!("The result is {counter}");
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn ternary_operator() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}