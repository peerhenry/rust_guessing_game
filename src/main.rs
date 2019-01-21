use std::num::ParseIntError;
use std::io;
use std::cmp::Ordering;
extern crate rand; // book puts this on top*
use rand::Rng;
// * I prefer dependency ordering: std, extern crates, own modules

fn main() {
  let mut rng = rand::thread_rng();
  let answer: i32 = rng.gen_range(1,100);  // telegram style naming :(
  try_with_message(answer, "Guess the number!");
}

fn try_with_message(answer: i32, msg: &str) {
  println!("{}", msg);
  guess(answer);
}

fn guess(answer: i32) {
  let mut guess = String::new();
  io::stdin().read_line(&mut guess)
    .expect("failed to read line");
  let guess_to_number_parse: Result<i32, ParseIntError> = guess.trim().parse();
  // 1. input must be trimmed to remove the newline character   
  // 2. using .unwrap() or .expect() may result in runtime panic => better to use result
  match guess_to_number_parse {
    Ok(guess_number) => {
      println!("you guessed: {}", guess);
      compare(answer, guess_number);
    },
    Err(_) => {
      if guess.trim() == "quit" || guess.trim() == "exit" { return; } 
      else { println!("Invalid input!"); }
      try_with_message(answer, "Please enter a valid number between 1 and 100!");
    }
  }
}

fn compare(answer: i32, guess: i32) { // compare
  match guess.cmp(&answer) { // match is better than if-else because of exhaustiveness checking
    Ordering::Less => try_with_message(answer, "Too small"),
    Ordering::Greater => try_with_message(answer, "Too small"),
    Ordering::Equal => println!("WINNING!")
  }
}
