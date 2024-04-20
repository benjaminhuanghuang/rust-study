use std::io::{self, BufRead, Write};

#[derive(Debug)]
enum State {
  Locked,
  Unlocked,
}

#[derive(Debug)]
enum Event {
  Coin,
  Push,
}

fn nextState(s: State, e: Event) -> State {
  match s {
    State::Locked => match e {
      Event::Coin => State::Unlocked,
      Event::Push => State::Locked,
    },
    State::Unlocked => match e {
      Event::Coin => State::Unlocked,
      Event::Push => State::Locked,
    },
  }
}

fn main() {
  let mut state = State::Locked;
  println!("> ");
  println!("State: {:?}", state);

  io::stdout().flush().unwrap();

  for line in io::stdin().lock().lines() {
    match line.unwrap().as_str() {
      "coin" => state = nextState(state, Event::Coin),
      "push" => state = nextState(state, Event::Push),
      unknown => {
        eprintln!("Invalid input {}", unknown);
      }
    };
    println!("State: {:?}", state);
    println!("> ");
    io::stdout().flush().unwrap();
  }
}
