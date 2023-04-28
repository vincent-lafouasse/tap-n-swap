#[derive(Debug)]
pub struct Hands {
    left_hand: usize,
    right_hand: usize,
}

impl Hands {
    pub fn new() -> Self {
        Hands {
            left_hand: 5,
            right_hand: 5,
        }
    }
}
