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
    from_symbol: Symbol,
    to_state: State,
    new_symbol: Symbol,
    head_move_dir: Direction,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
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
    transition_table: HashMap<u32, HashMap<Symbol, TransitionRule>>,
    __visible_area: (usize, usize)
}

impl TuringMachine {
    pub fn new() -> TuringMachine {
        TuringMachine {
            tape: vec![0; DEFAULT_TAPE_SIZE as usize],
            initial_state: None,
            head: DEFAULT_TAPE_SIZE / 2 * USIZE_BIT_SIZE, // set the head to the middle of the tape by default
            states: HashMap::default(),
            transition_table: HashMap::default(),
            __visible_area: (0, 0),
        }
    }

    pub fn run(&mut self) -> Result<State, String> {
        let initial_state = self.states.get(
            &self.initial_state.ok_or("ERROR: initial state is not set")?,
        ).unwrap();

        let mut current_state = State::ProgramState(*initial_state);
        
        loop {
            match current_state {
                State::ProgramState(ProgramState { id: state_id }) => {
                    let current_symbol = self.get_head_value();
                    let transition_rule = self.get_transition_rule(&state_id, &current_symbol);
                    match transition_rule {
                        Some(TransitionRule { to_state, new_symbol, head_move_dir, .. }) => {
                            let new_symbol = *new_symbol;
                            let head_move_dir = *head_move_dir;
                            current_state = *to_state;
                            self.set_head_value(new_symbol);
                            self.move_head(head_move_dir);
                        }
                        None => return Ok(State::Halt),
                    }
                },
                state => return Ok(state)
            }
        }
    }

    pub fn get_transition_rule(&self, state_id: &u32, symbol: &Symbol) -> Option<&TransitionRule> {
        self.transition_table
            .get(&state_id).unwrap()
            .get(symbol)
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
            let from_state = &t.from_state;
            let from_symbol = &t.from_symbol;
            if !self.transition_table.contains_key(from_state) {
                self.transition_table.insert(*from_state, HashMap::default());
            }
            self.transition_table
                .get_mut(from_state).unwrap()
                .insert(*from_symbol, *t);
        }

        Ok(())
    }

    fn validate_transition_rules(&self, transition_rules: &Vec<TransitionRule>) -> Result<(), String> {
        let mut states_used = HashMap::<&u32, Vec<Symbol>>::new();

        for t in transition_rules {
            let from_state = &t.from_state;
            let from_symbol = &t.from_symbol;
            if !self.states.contains_key(from_state) {
                return Err(format!("ERROR: State with id `{}` does not exist", from_state));
            }
            
            if !states_used.contains_key(from_state) {
                states_used.insert(from_state, Vec::new());
            }
            
            let already_mapped_symbols = states_used.get_mut(from_state).unwrap();
            if already_mapped_symbols.contains(from_symbol) {
                return Err(format!("ERROR: State with id `{}` is already bound to a transition rule as a `from_state`", from_state));
            }
            already_mapped_symbols.push(*from_symbol);
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

    pub fn move_head(&mut self, direction: Direction) {
        // TODO: if head moves outside tape bounds, reallocate tape with double size
        let delta = match direction {
            Direction::Left(delta) => delta,
            Direction::Right(delta) => delta,
        };
        self.head = self.head + delta as usize;
    }

    pub fn set_head_value(&mut self, value: Symbol) {
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

// TODO: add tests
// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn new_test() {
        
//     }
// }