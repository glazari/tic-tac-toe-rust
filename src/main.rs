use std::cmp::PartialOrd;

fn main() {
    let mut b = Board::new();
    b.b[0][1] = 'o';
    b.b[1][1] = 'x';
    b.print();
}

struct Board {
    b: [[char; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board { b: [[' '; 3]; 3] }
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

enum XOs {
    X,
    O,
    Empty,
}
