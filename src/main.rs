use turing_machine::{ ProgramState, State, TuringMachine };

fn main() {
    let mut machine = TuringMachine::new();

    match machine.run() {
        Err(err) => println!("{}", err),
        Ok(finish_state) => match finish_state {
            State::Halt => println!("Machine halted"),
            State::Termination => println!("Machine terminated"),
            State::ProgramState(ProgramState { id }) => println!("Machine stopped with invalid state with id `{}`", id),
        }
    }
}
