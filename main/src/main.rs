use std::io::{self,Write};

enum State { // to be filled in
    StateA,
    StateB,
    StateC,
    StateD,
    StateE,
}

impl State {
    fn next(self) -> State {
        match self { // to be filled in
            State::StateA => State::StateB,
            State::StateB => State::StateC,
            State::StateC => State::StateD,
            State::StateD => State::StateE,
            State::StateE => State::StateA,
        }
    }
}

fn main() {
    let mut currentState = State::StateA; // starting state
    // should be customisable

    loop {
        
    }
}