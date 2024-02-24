mod turing_machine;

use crate::turing_machine::TuringMachine;

fn main() {
    let machine = TuringMachine::new();

    print!("{}\n", machine.pointer());
    machine.print_tape();
}
