pub const USIZE_BIT_SIZE: usize = usize::BITS as usize;

// TODO: rewrite with macros
pub fn get_bit(cell: &usize, index: &usize) -> usize {
    (cell >> (USIZE_BIT_SIZE - index - 1)) & 1 
}

pub fn set_bit(cell: &mut usize, index: &usize) {
    *cell |= 1 << (USIZE_BIT_SIZE - index - 1);
}

pub fn unset_bit(cell: &mut usize, index: &usize) {
    *cell &= !(1 << (USIZE_BIT_SIZE - index - 1))
}
