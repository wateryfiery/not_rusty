#![allow(unused)]
use std::fs::File;

/*
 * example1:
 *   let greater: u8 = return_greater(10, 11);
 *   println!("{}", greater);
 *
 * data will be passed to a function either:
 *  1) by value = the actual value
 *  2) by reference = the pointer to the value
 *
 *
 * example2:
 *  remember a string is on the heap, so pass by reference (use ampersand sign)
 *
 *   let mut original: String = String::from("original value");
 *   println!("\noriginal (outer scope): \t\"{}\"", original); 
     {
         print_original(&original);
         change_original(&mut original);
         println!("original (inner scope): \t\"{}\"", original);
     }
 *
 * ____
 *
 * Closures (anonymous functions):
 *  functions without names
 *  do more research
 *
 *  ____
 *
 * Error handling:
 *  2 types:
 *      recoverable = continues to run
 *      unrecoverable = program panics and crashes
 *  2 ways to handle errors:
 *      1) handle the error
 *      2) propagate the error = like log error and move on
 *
 *
 *
 *
 *
 *
 *
 * 
 */
fn main() {
    println!("cool");
}

fn print_original(original: &String) {
    println!("fn print_original: \t\"{}\"", original);
}

fn change_original(original: &mut String) {
    let next: &mut String = original;
    *next = String::from("next value");
    println!("fn change_original: \t\"{}\"", original);
}

fn not_a_closure() {
    let name: &str = "dylan george";
    let slogan: &str = "life is too short to be serious about 80% of things";
    println!("{}\n{}", name, slogan);
}


