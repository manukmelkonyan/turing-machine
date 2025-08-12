use turing_machine::{ Direction, ProgramState, State, Symbol, TransitionRule, TuringMachine };

fn main() {
    let mut machine = TuringMachine::new();

    let q1 = ProgramState{id: 1};
    let q2 = ProgramState{id: 2};
    let q3 = ProgramState{id: 3};
    let q4 = ProgramState{id: 4};

    machine.define_states(&vec![ q1,q2,q3,q4 ]);
    
    machine.set_initial_state(q1.id).expect("Initial state is not set");
    
    machine.define_transition_table(&vec![
        TransitionRule::new(q1, Symbol::Zero, Symbol::Zero, Direction::Stay, State::Termination),
        TransitionRule::new(q1, Symbol::One, Symbol::Zero, Direction::Right, State::ProgramState(q2)),

        TransitionRule::new(q2, Symbol::Zero, Symbol::One, Direction::Left, State::ProgramState(q3)),
        TransitionRule::new(q2, Symbol::One, Symbol::One, Direction::Right, State::ProgramState(q2)),

        TransitionRule::new(q3, Symbol::Zero, Symbol::Zero, Direction::Right, State::ProgramState(q4)),
        TransitionRule::new(q3, Symbol::One, Symbol::One, Direction::Left, State::ProgramState(q3)),

        TransitionRule::new(q4, Symbol::Zero, Symbol::Zero, Direction::Stay, State::Halt),
        TransitionRule::new(q4, Symbol::One, Symbol::Zero, Direction::Right, State::Termination),
    ]).unwrap();

    let input = vec![
        Symbol::One, Symbol::One, Symbol::One, Symbol::One, // 3
        Symbol::Zero,
        Symbol::One, Symbol::One, Symbol::One, // 2
    ];
    
    machine.write_to_tape(&input);
    
    println!("Initial tape:");
    machine.print_tape();
    println!("--------------------------------");
    
    match machine.run() {
        Err(err) => println!("Error: {}", err),
        Ok(finish_state) => match finish_state {
            State::Halt => println!("Machine halted"),
            State::Termination => {
                println!("Machine terminated");
                println!("Final tape:");
                machine.print_tape();
            },
            State::ProgramState(ProgramState { id }) => println!("Machine stopped with invalid state with id `{}`", id),
        }
    }
}
