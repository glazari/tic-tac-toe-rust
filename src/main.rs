use std::cmp::PartialOrd;

fn main() {
    let mut b = Board::new();
    b.b[0][1] = Move::O;
    b.b[1][1] = Move::X;
    b.print();
}

struct Board {
    b: [[Move; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            b: [[Move::free; 3]; 3],
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

    fn play_x(&self, i: usize, j: usize) {}
}

#[derive(Copy, Clone)]
enum Move {
    X,
    O,
    free,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Move::X => write!(f, "X"),
            Move::O => write!(f, "O"),
            Move::free => write!(f, " "),
        }
    }
}
