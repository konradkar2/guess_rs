#![feature(random)]
use std::cmp::Ordering;
// requires nightly toolchain
use std::io::{self, BufRead, StdinLock, Write, stdout};
use std::random::random;

fn read_number(stdin: &mut StdinLock<'static>) -> Option<i32> {
    let mut input = String::new();

    stdin.read_line(&mut input).expect("reading input works");

    let number = input.trim().parse::<i32>();

    match number {
        Ok(number) => return Some(number),
        Err(..) => return None,
    }
}

fn main() {
    println!("Hello, give me min and max number to guess from");

    let mut stdin = io::stdin().lock();

    let mut ask_for_number = |text: &str| -> i32 {
        let mut number: Option<i32> = None;
        while number == None {
            print!("{}", text);
            stdout().flush().unwrap();
            number = read_number(&mut stdin);
        }

        return number.unwrap();
    };

    let num_min = ask_for_number("give me min: ");
    let num_max = ask_for_number("give me max: ");

    if num_min > num_max {
        panic!("mf provide valid input, max is smaller than min");
    }

    let random_number = num_min + (random::<u32>() % (num_max - num_min + 1) as u32) as i32;

    let max_attemps = 4;
    println!("randomized the number, you have {} attemps", max_attemps);
    let mut attempts = 0;
    let mut guess = random_number - 1;

    while guess != random_number && attempts < max_attemps {
        attempts += 1;
        guess = ask_for_number("take a guess: ");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("it's too little!"),
            Ordering::Greater => println!("it's too big!"),
            Ordering::Equal => println!(
                "you've guessed the number after {} attempts, the number is {}",
                attempts, random_number
            ),
        }
    }

    if guess != random_number {
        println!(
            "you failed to guess the number! The number was {}",
            random_number
        )
    }
}
