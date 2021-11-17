fn main() {
    let mut b = Board::new();
    b.play_o(0, 1);
    b.play_x(1, 1);
    b.print();
}

struct Board {
    b: [[Move; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            b: [[Move::Free; 3]; 3],
        }
    }

    fn print(&self) {
        for i in 0..3 {
            for j in 0..3 {
                print!(" {} ", self.b[i][j]);
                if j < 2 {
                    print!("|");
                }
            }
            if i < 2 {
                print!("\n___|___|___");
            } else {
                print!("\n   |   |   ");
            }
            println!();
        }
        println!();
    }

    fn play_x(&mut self, i: usize, j: usize) {
        self.b[i][j] = Move::X;
    }
    fn play_o(&mut self, i: usize, j: usize) {
        self.b[i][j] = Move::O;
    }
}

#[derive(Copy, Clone)]
enum Move {
    X,
    O,
    Free,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Move::X => write!(f, "X"),
            Move::O => write!(f, "O"),
            Move::Free => write!(f, " "),
        }
    }
}
