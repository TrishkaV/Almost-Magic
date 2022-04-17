mod square_methods;
use square_methods::{
    populate_first_square, populate_fourth_square, populate_second_square, populate_third_square,
};
#[cfg(debug_assertions)]
use std::env;
#[cfg(not(debug_assertions))]
use std::io::prelude::*;
#[cfg(not(debug_assertions))]
use std::io::stdin;

fn main() {
    #[cfg(debug_assertions)]
    env::set_var("RUST_BACKTRACE", "1");
    #[cfg(not(debug_assertions))]
    print_welcome_message();

    let mut total_sum: u32 = 0;
    let mut first_square: [u32; 9];
    let mut second_square: Option<[u32; 9]>;
    let mut third_square: Option<[u32; 9]>;
    let mut fourth_square: Option<[u32; 9]>;
    while total_sum >= 1111 || total_sum == 0 {
        if total_sum >= 1111 {
            println!("Total sum  --> {}, is too high. Retrying...", total_sum);
        }
        total_sum = 0;
        first_square = populate_first_square();
        second_square = get_second(first_square);
        if second_square == None {
            continue;
        }
        third_square = get_third(first_square, second_square);
        if third_square == None {
            continue;
        }
        fourth_square = get_fourth(first_square, third_square);
        if fourth_square == None {
            continue;
        }

        let all_squares: Vec<[u32; 9]> = vec![
            first_square,
            second_square.unwrap(),
            third_square.unwrap(),
            fourth_square.unwrap(),
        ];
        for square in all_squares {
            for i in square {
                total_sum += i;
            }
        }
        println!("\n\nSquare n.1 --> {:?}", first_square);
        println!("Square n.2 --> {:?}", second_square.unwrap());
        println!("Square n.3 --> {:?}", third_square.unwrap());
        println!("Square n.4 --> {:?}", fourth_square.unwrap());
    }
    println!("Total sum  --> {}, OK!\n", total_sum);
    #[cfg(not(debug_assertions))]
    print_goodbye_message();
}

fn get_second(first: [u32; 9]) -> Option<[u32; 9]> {
    loop {
        match populate_second_square(first, 1) {
            Ok(returned_square) => return Some(returned_square),
            Err(_e) => return None,
        }
    }
}

fn get_third(first: [u32; 9], second: Option<[u32; 9]>) -> Option<[u32; 9]> {
    loop {
        match populate_third_square(second.unwrap(), 2, first) {
            Ok(returned_square) => return Some(returned_square),
            Err(_e) => return None,
        }
    }
}

fn get_fourth(first: [u32; 9], third: Option<[u32; 9]>) -> Option<[u32; 9]> {
    loop {
        match populate_fourth_square(third.unwrap(), first) {
            Ok(returned_square) => return Some(returned_square),
            Err(_e) => return None,
        }
    }
}

#[cfg(not(debug_assertions))]
fn print_welcome_message() {
    println!(
        "\nHello, this is an \"Almost magic square\" puzzle solver 
        (as outlined here: https://tinyurl.com/almost-magic-puzzle)."
    );
    println!("\nThe target is set to a total elements sum of < 1111.");
    println!("\nPress enter to begin...");
    let _a = stdin().lock().read_line(&mut String::new());
    println!("Running...");
}

#[cfg(not(debug_assertions))]
fn print_goodbye_message() {
    println!("\nThank you for using this software.\n\nPress enter to close this window...");
    let _a = stdin().lock().read_line(&mut String::new());
}
