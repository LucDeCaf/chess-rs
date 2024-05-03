struct Board {
    bitboards: [i64; 6], // 6 is a random number idk
}

impl Board {
    fn new() -> Self {
        Self { bitboards: [0; 6] }
    }
}

fn main() {
    let mut board = Board::new();
}
