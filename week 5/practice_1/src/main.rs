// Rust program to output name and age

use std::io;

fn main () {
    println!("\nStudent Information Management System!");

   // input name
   println!("\nPlease Enter your name.");
   let mut name = String::new();
       io::stdin()
       .read_line(&mut name)
       .expect("failed to input");
    println!("your name is {}", name);

    // input age
    println!("\nEnter your age.");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("failed to read input");
    let age:i32 = age.trim().parse().expect("input not an intrger");
    println!("your age is: {}",age);   
}
