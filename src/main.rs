use std::io;

fn main() {
  let number = 5;
  retry_with_message(number, "Guess the number!");
}

fn guess(number: u32) {
  let mut guess = String::new();
  io::stdin().read_line(&mut guess)
    .expect("failed to read line");
  println!("you guessed: {}", guess);
}

fn retry_with_message(number: u32, msg: &str) {
  println!("{}", msg);
  guess(number);
}
