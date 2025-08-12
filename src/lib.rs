pub mod bit_vec;
use bit_vec::{USIZE_BIT_SIZE, get_bit, set_bit, unset_bit};
use std::collections::{HashMap};

const DEFAULT_TAPE_SIZE: usize = 2; // this is not the actual tape size (number of bit-vectors)

type ProgramStateId = u32;

#[derive(Clone, Copy)]
pub enum Direction {
    Left = -1,
    Right = 1,
    Stay = 0,
}

#[derive(Clone, Copy)]
pub struct TransitionRule {
    pub from_state: ProgramState,
    pub from_symbol: Symbol,
    pub to_state: State,
    pub new_symbol: Symbol,
    pub head_move_dir: Direction,
}

impl TransitionRule {
    pub fn new(from_state: ProgramState, from_symbol: Symbol, new_symbol: Symbol, head_move_dir: Direction, to_state: State) -> TransitionRule {
        TransitionRule { from_state, from_symbol, new_symbol, head_move_dir, to_state }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Symbol {
    Zero = 0,
    One = 1,
}

impl Symbol {
    pub fn vec_from_numbers(numbers: &[u8]) -> Vec<Symbol> {
        numbers
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
    pub id: ProgramStateId,
}

#[derive(Clone, Copy)]
pub enum State {
    ProgramState(ProgramState),
    Termination,
    Halt,
}

impl State {
    pub fn define(id: ProgramStateId) -> State {
        State::ProgramState(
            ProgramState { id }
        )
    }
}

pub struct TuringMachine {
    tape: Vec<usize>, // bit-vector tape
    head: usize,
    initial_state: Option<ProgramStateId>,
    states: HashMap<ProgramStateId, ProgramState>,
    transition_table: HashMap<ProgramStateId, HashMap<Symbol, TransitionRule>>,
    __visible_area: (usize, usize)
}

impl TuringMachine {
    pub fn new() -> TuringMachine {
        TuringMachine {
            tape: vec![0; DEFAULT_TAPE_SIZE as usize],
            initial_state: None,
            head: DEFAULT_TAPE_SIZE / 2 * USIZE_BIT_SIZE, // set the head to the center of the tape by default
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
                    print!("q{}: ", state_id);
                    self.print_tape();
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

    pub fn get_transition_rule(&self, state_id: &ProgramStateId, symbol: &Symbol) -> Option<&TransitionRule> {
        self.transition_table
            .get(&state_id).unwrap()
            .get(symbol)
    }

    pub fn set_initial_state(&mut self, state_id: ProgramStateId) -> Result<(), String> {
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
            if !self.transition_table.contains_key(&from_state.id) {
                self.transition_table.insert(from_state.id, HashMap::default());
            }
            self.transition_table
                .get_mut(&from_state.id).unwrap()
                .insert(*from_symbol, *t);
        }

        Ok(())
    }

    fn validate_transition_rules(&self, transition_rules: &Vec<TransitionRule>) -> Result<(), String> {
        let mut states_used = HashMap::<&ProgramStateId, Vec<Symbol>>::new();

        for t in transition_rules {
            let from_state = &t.from_state;
            let from_symbol = &t.from_symbol;
            if !self.states.contains_key(&from_state.id) {
                return Err(format!("ERROR: State with id `{}` does not exist", from_state.id));
            }
            
            if !states_used.contains_key(&from_state.id) {
                states_used.insert(&from_state.id, Vec::new());
            }
            
            let already_mapped_symbols = states_used.get_mut(&from_state.id).unwrap();
            if already_mapped_symbols.contains(from_symbol) {
                return Err(format!("ERROR: State with id `{}` is already bound to a transition rule as a `from_state`", from_state.id));
            }
            already_mapped_symbols.push(*from_symbol);
        }
        Ok(())
    }

    pub fn write_to_tape(&mut self, cells: &[Symbol]) {
        // TODO: add check for tape size and dynamically reallocate tape if needed
        assert!(cells.len() > self.tape.len(), "The length of the cells to be written to the tape should be less than the tape size");
        
        cells
            .chunks(USIZE_BIT_SIZE)
            .enumerate()
            .for_each(|(i, chunk)| {
                let current_head = self.head + i * USIZE_BIT_SIZE;
                let cell = &mut self.tape[current_head / USIZE_BIT_SIZE];

                chunk.iter().enumerate().for_each(|(j, symbol)| {
                    let bit_idx = self.head % USIZE_BIT_SIZE + j;
                    match symbol {
                        Symbol::Zero => unset_bit(cell, &bit_idx),
                        Symbol::One => set_bit(cell, &bit_idx),
                    }
                });
            }
        );
    }

    pub fn head(&self) -> usize { self.head }

    pub fn print_tape_observed_area(&self, offset: Option<usize>) {
        let offset = offset.unwrap_or(0);
        let start = {
            let first_non_zero_idx = self.tape.iter().position(|&x| x != 0).unwrap_or(0) as isize - offset as isize;
            first_non_zero_idx.max(0) as usize
        };
        let last_non_zero_idx = {
            let last_non_zero_idx = self.tape.iter().rposition(|&x| x != 0).unwrap_or(self.tape.len() - 1) + offset;
            last_non_zero_idx.min(self.tape.len() - 1)
        };
        let observed_area = &self.tape[start..last_non_zero_idx];

        observed_area.iter().for_each(|cell| {
            print!("{:032b}", cell);
        });
        println!();
    }

    pub fn print_tape(&self) {
        let binary_str = self.tape
            .iter()
            .fold(String::new(), |mut acc, item| {
                acc.push_str(format!("{:0width$b}", item, width = USIZE_BIT_SIZE).as_str());
                acc
            });

        let head_val = self.get_head_value();
        let binary_str = format!(
            "{prefix}\x1b[32m\x1b[4m{head_val}\x1b[0m{postfix}",
            prefix = &binary_str[0..self.head],
            head_val = head_val as u8,
            postfix = &binary_str[self.head + 1..],
        );
        
        println!("{}", binary_str);
    }

    pub fn tape_len(&self) -> usize {
        (self.tape.len() * USIZE_BIT_SIZE) as usize
    }

    pub fn get_head_value(&self) -> Symbol {
        // 9 <=> 1.0

        let cell = &self.tape[self.head / USIZE_BIT_SIZE];
        let bit_idx = self.head % USIZE_BIT_SIZE;

        let value = get_bit(cell, &bit_idx);
        match value {
            0 => Symbol::Zero,
            1 => Symbol::One,
            _ => panic!("Unexpected bit value: {}", value),
        }
    }

    pub fn move_head(&mut self, direction: Direction) {
        // TODO: if head moves outside tape bounds, reallocate tape with double size
        self.head = (self.head as isize + direction as isize) as usize;
    }

    pub fn set_head_value(&mut self, value: Symbol) {
        let cell = &mut self.tape[self.head / USIZE_BIT_SIZE];
        let bit_idx = self.head % USIZE_BIT_SIZE;

        match value {
            Symbol::Zero => unset_bit(cell, &bit_idx),
            Symbol::One => set_bit(cell, &bit_idx),
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