use std::collections::{HashMap, HashSet};

const USIZE_BIT_SIZE: usize = usize::BITS as usize;
const DEFAULT_TAPE_SIZE: usize = 1000; // not the bitvec length.

#[derive(Clone, Copy)]
#[repr(i8)]
pub enum Direction {
    Left(i8) = -1,
    Right(i8) = 1,
}

#[derive(Clone, Copy)]
pub struct TransitionRule {
    from_state: u32,
    to_state: State,
    read_symbol: Symbol,
    write_symbol: Symbol,
    head_direction: Direction,
}

#[derive(Clone, Copy)]
pub enum Symbol {
    Zero = 0,
    One = 1,
}

impl Symbol {
    fn vec_from_numbers(nums: &[u8]) -> Vec<Symbol> {
        nums
            .iter()
            .map(|num| {
                match num {
                    0 => Symbol::Zero,
                    1 => Symbol::One,
                    _ => panic!("Unexpected bit value: {}", num),
                }
            })
            .collect::<Vec<Symbol>>()
    }
}

#[derive(Clone, Copy)]
pub struct ProgramState {
    pub id: u32,
}

#[derive(Clone, Copy)]
pub enum State {
    ProgramState(ProgramState),
    Termination,
    Halt,
}

impl State {
    pub fn define(id: u32) -> State {
        State::ProgramState(
            ProgramState { id }
        )
    }
}

// TODO: rewrite with macros
pub mod bit_vec {
    use super::USIZE_BIT_SIZE;

    pub fn get_bit(cell: &usize, index: &usize) -> usize {
        (cell >> (USIZE_BIT_SIZE - index - 1)) & 1 
    }

    pub fn set_bit(cell: &mut usize, index: &usize) {
        *cell |= 1 << (USIZE_BIT_SIZE - index - 1);
    }

    pub fn unset_bit(cell: &mut usize, index: &usize) {
        *cell &= !(1 << (USIZE_BIT_SIZE - index - 1))
    }
}

pub struct TuringMachine {
    tape: Vec<usize>, // bit-vector tape
    head: usize,
    initial_state: Option<u32>,
    states: HashMap<u32, ProgramState>,
    transition_table: Vec<TransitionRule>,
    __visible_area: (usize, usize)
}

impl TuringMachine {
    pub fn new() -> TuringMachine {
        TuringMachine {
            tape: vec![0; DEFAULT_TAPE_SIZE as usize],
            initial_state: None,
            head: DEFAULT_TAPE_SIZE / 2 * USIZE_BIT_SIZE, // set the head to the middle of the tape by default
            states: HashMap::default(),
            transition_table: Vec::default(),
            __visible_area: (0, 0),
        }
    }

    pub fn run(&mut self) -> Result<State, String> {
        let initial_state = self.initial_state.ok_or("ERROR: initial state is not set");
        
        Ok(State::Halt)
    }

    pub fn set_initial_state(&mut self, state_id: u32) -> Result<(), String> {
        if !self.states.contains_key(&state_id) {
            return Err(format!("ERROR: state with id `{}` is not defined", &state_id));
        }
        self.initial_state = Some(state_id);
        Ok(())
    }

    pub fn define_states(&mut self, program_states: &Vec<ProgramState>, ) {
        program_states.iter().for_each(|state| {
            self.states.insert(state.id, *state);
        });
    }

    pub fn define_transition_table(&mut self, transition_rules: &Vec<TransitionRule>) -> Result<(), String> {
        self.validate_transition_rules(transition_rules)?;
        
        for t in transition_rules {
            self.transition_table.push(*t);
        }

        Ok(())
    }

    fn validate_transition_rules(&self, transition_rules: &Vec<TransitionRule>) -> Result<(), String> {
        let mut states_used = HashSet::<&u32>::new();

        for t in transition_rules {
            if !self.states.contains_key(&t.from_state) {
                return Err(format!("ERROR: State with id `{}` does not exist", &t.from_state));
            }
            if states_used.contains(&t.from_state) {
                return Err(format!("ERROR: State with id `{}` is already bound to a transition rule as a `from_state`", &t.from_state));
            }
            states_used.insert(&t.from_state);
        }
        Ok(())
    }

    pub fn write_to_tape(mut self, cells: &[Symbol]) {
        assert!(cells.len() < self.tape.len(), "The length of the cells to be written to the tape should be less than the tape size");
        
        cells
            .chunks(USIZE_BIT_SIZE)
            .enumerate()
            .for_each(|(i, chunk)| {
                let current_head = self.head + i * USIZE_BIT_SIZE;
                let cell = &mut self.tape[current_head / USIZE_BIT_SIZE];

                chunk.iter().for_each(|symbol| {
                    let bit_idx = self.head % USIZE_BIT_SIZE;
                    match symbol {
                        Symbol::Zero => bit_vec::unset_bit(cell, &bit_idx),
                        Symbol::One => bit_vec::set_bit(cell, &bit_idx),
                    }
                });
            }
        );
    }

    pub fn head(&self) -> usize { self.head }

    pub fn print_tape(&self) {
        let binary_str = self.tape
            .iter()
            .fold(String::new(), |mut acc, item| {
                acc.push_str(format!("{:032b}", item).as_str());
                acc
            });
        println!("{}", binary_str);
    }

    pub fn tape_len(&self) -> usize {
        (self.tape.len() * 32) as usize
    }

    pub fn get_head_value(&self) -> Symbol {
        // 9 <=> 1.0

        let cell = &self.tape[self.head / USIZE_BIT_SIZE];
        let bit_idx = self.head % USIZE_BIT_SIZE;

        let value = bit_vec::get_bit(cell, &bit_idx);
        match value {
            0 => Symbol::Zero,
            1 => Symbol::One,
            _ => panic!("Unexpected bit value: {}", value),
        }
    }

    pub fn set_head_value(&mut self, value: &Symbol) {
        let cell = &mut self.tape[self.head / USIZE_BIT_SIZE];
        let bit_idx = self.head % USIZE_BIT_SIZE;

        match value {
            Symbol::Zero => bit_vec::unset_bit(cell, &bit_idx),
            Symbol::One => bit_vec::set_bit(cell, &bit_idx),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////// TESTS ////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn new_test() {
        
//     }
// }