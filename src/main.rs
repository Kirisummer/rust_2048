use std::io;

mod matrix;
mod field;
use field::Field;
use field::ShiftDim;
use field::ShiftDir;

enum Action {
    Shift(ShiftDim, ShiftDir),
    Quit,
}

fn get_action(stdin: &io::Stdin) -> Action {
    let mut action = String::new();
    loop {
        stdin.read_line(&mut action).expect("Failed to read action from stdin");
        if let Some(action) = select_action(action.trim()) {
            return action;
        } else {
            action.clear();
        }
    }
}

fn select_action(s: &str) -> Option<Action> {
    match s {
        "a" | "h" => Some(Action::Shift(ShiftDim::Horizontal, ShiftDir::Direct)),
        "d" | "l" => Some(Action::Shift(ShiftDim::Horizontal, ShiftDir::Reverse)),
        "w" | "k" => Some(Action::Shift(ShiftDim::Vertical,   ShiftDir::Direct)),
        "s" | "j" => Some(Action::Shift(ShiftDim::Vertical,   ShiftDir::Reverse)),
        "q" => Some(Action::Quit),
        _   => None
    }
}

fn main() {
    let size = 4;
    let mut field = Field::new(size);
    let mut score = 0;
    let mut move_again = false;
    let stdin = io::stdin();

    while move_again || field.add_random_cell() {
        println!("{}", field);
        println!("Total score: {}", score);

        match get_action(&stdin) {
            Action::Shift(dim, dir) => {
                if let Some(points) = field.shift(dim, dir) {
                    move_again = false;
                    score += points;
                    println!("Got {} points", points);
                } else {
                    move_again = true;
                    println!("No move");
                }
            }
            Action::Quit => break,
        }
    }

    println!("Game over! Final score: {}", score);
}

