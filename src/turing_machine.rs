const DEFAULT_TAPE_SIZE: usize = 1000;

pub enum Action {
    Left,
    Right,
    Stay,
}

pub enum Symbol {
    ZERO,
    ONE,
}

// TODO
// pub struct State {
//     action: Action
// }

pub struct TuringMachine {
    tape: Vec<usize>, // each item is syzeof(usize) length bitvec
    pointer: usize,
    __visible_area: (usize, usize)
}

impl TuringMachine {
    pub fn new() -> TuringMachine {
        
        TuringMachine {
            tape: vec![0; DEFAULT_TAPE_SIZE as usize],
            pointer: DEFAULT_TAPE_SIZE / 2,
            __visible_area: (0, 0)
        }
    }

    pub fn write_to_tape(&mut self, cells: Vec<u32>, pointer_idx: u32) {
        let length = cells.len();
        
    }

    pub fn pointer(&self) -> usize { self.pointer }

    pub fn print_tape(&self) {
        let binary_str = self.tape
            .iter()
            .fold(String::new(), |mut acc, item| {
                acc.push_str(format!("{:032b}", item).as_str());
                acc
            });
        print!("{}", binary_str);
    }

    pub fn tape_len(&self) -> u32 {
        (self.tape.len() * 32) as u32
    }

    pub fn get_pointing_value(&self) -> Symbol {
        std::mem::size_of::<usize>();
        
        Symbol::ONE
    }
}