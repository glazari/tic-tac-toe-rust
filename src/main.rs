fn main() {
    let mut b = Board::new();
    b.play_o(0, 1);
    b.play_x(1, 1);
    b.print();
}

struct Board {
    b: [[Option<Player>; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board { b: [[None; 3]; 3] }
    }

    fn print(&self) {
        for i in 0..3 {
            for j in 0..3 {
                match self.b[i][j] {
                    None => print!("   "),
                    Some(p) => print!(" {} ", p),
                }
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
        self.b[i][j] = Some(Player::X);
    }
    fn play_o(&mut self, i: usize, j: usize) {
        self.b[i][j] = Some(Player::O);
    }
}

#[derive(Copy, Clone)]
enum Player {
    X,
    O,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}
