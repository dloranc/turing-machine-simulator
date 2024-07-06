use std::usize::MAX;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    machine: String,

    /// Name of the file to process
    #[arg(short, long)]
    filename: Option<String>,
}

struct Machine {
    state_count: usize,
    transition_count: usize,
    states: Vec<Vec<Transition>>,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Transition {
    to_write: usize,
    direction: Direction,
    to_state: usize,
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Halt,
}

fn main() {
    let args = Args::parse();

    match &args.filename {
        Some(filename) => {
            println!("{}", filename);
        }
        None => {}
    }

    if !args.machine.is_empty() {
        println!("Machine definition: {}", args.machine);

        let machine = create_machine(args.machine);

        print_machine(machine);
    }
}

fn split_ascii_string(s: &str, chunk_size: usize) -> Vec<String> {
    s.as_bytes()
        .chunks(chunk_size)
        .map(|chunk| std::str::from_utf8(chunk).unwrap().to_string())
        .collect()
}

fn create_machine(machine: String) -> Machine {
    let states: Vec<&str> = machine.split("_").collect();

    let state_count = states.len();
    let transition_count = states[0].len() / 3;

    let mut machine = Machine {
        transition_count,
        state_count,
        states: Vec::with_capacity(state_count),
    };

    for (i, state) in states.iter().enumerate() {
        let states = split_ascii_string(state, 3);
        machine.states.push(Vec::new());

        for transition in states {
            machine.states[i].push(Transition {
                to_write: match transition.chars().nth(0) {
                    Some('-') => 0,
                    Some(c) => c as usize - '0' as usize,
                    None => panic!("error: nothing to write"),
                },
                direction: match transition.chars().nth(1) {
                    Some('-') => Direction::Halt,
                    Some('L') => Direction::Left,
                    Some('R') => Direction::Right,
                    _ => panic!("error: no direction"),
                },
                to_state: match transition.chars().nth(2) {
                    Some('-') => MAX,
                    Some(c) => c as usize - 'A' as usize,
                    None => panic!("error: no no state"),
                },
            })
        }
    }

    return machine;
}

fn print_machine(machine: Machine) {
    match machine.transition_count {
        2 => println!("BB({})", machine.state_count),
        _ => println!("BB({}, {})", machine.state_count, machine.transition_count),
    }

    for state in machine.states {
        for transition in state {
            println!("To write: {}", transition.to_write);
            println!("Direction: {:?}", transition.direction);
            println!("To state: {}", transition.to_state);
        }
        println!();
    }
}

#[test]
fn test_create_simple_two_state_machine_with_two_symbols() {
    let machine = create_machine("0RB1RB_0LA---".to_string());

    assert_eq!(machine.state_count, 2);
    assert_eq!(machine.transition_count, 2);

    assert_eq!(machine.states[0][0], Transition { to_write: 0, direction: Direction::Right, to_state: 1 });
    assert_eq!(machine.states[0][1], Transition { to_write: 1, direction: Direction::Right, to_state: 1 });
    assert_eq!(machine.states[1][0], Transition { to_write: 0, direction: Direction::Left, to_state: 0 });
    assert_eq!(machine.states[1][1], Transition { to_write: 0, direction: Direction::Halt, to_state: MAX });
}
