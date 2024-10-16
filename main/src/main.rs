use std::io::{self, Write};

trait State {
    fn next(self: Box<Self>) -> Box<dyn State>;
    fn print_state(&self);
}

struct State1;

impl State for State1 {
    fn next(self: Box<Self>) -> Box<dyn State> {
        Box::new(State2)
    }

    fn print_state(&self) {
        println!("Current State: State 1");
    }
}

struct State2;

impl State for State2 {
    fn next(self: Box<Self>) -> Box<dyn State> {
        Box::new(State3) 
    }

    fn print_state(&self) {
        println!("Current state: State2");
    }
}

struct State3;

impl State for State3 {
    fn next(self: Box<Self>) -> Box<dyn State> {
        Box::new(State4) 
    }

    fn print_state(&self) {
        println!("Current state: State3");
    }
}

struct State4;

impl State for State4 {
    fn next(self: Box<Self>) -> Box<dyn State> {
        Box::new(State5) 
    }

    fn print_state(&self) {
        println!("Current state: State4");
    }
}

struct State5;

impl State for State5 {
    fn next(self: Box<Self>) -> Box<dyn State> {
        Box::new(State1) 
    }

    fn print_state(&self) {
        println!("Current state: State5");
    }
}

fn main() {
    let mut current_state : Box<dyn State> = Box::new(State1);

    loop {
        current_state.print_state();

        print!("Press 's' to switch to the next state or 'q' to quit: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "s" => {
                current_state = current_state.next();
            }
            "q" => {
                println!("Exiting the program");
                break;
            }
            _ => {
                println!("Invalid input. Please follow the instructions.");
            }
        }
    }
}