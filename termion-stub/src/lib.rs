// Stub implementation of termion for Windows compatibility

pub mod color {
    pub struct Fg<C>(pub C);
    pub struct Reset;
    
    pub const Red: u8 = 0;
}

pub mod style {
    pub struct Reset;
    pub struct Bold;
}

pub fn is_tty<T: ?Sized>(_: &T) -> bool {
    false
}
