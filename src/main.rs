use std::io;
use std::io::*;

fn main() {
    let mut b = Board::new();

    b.print();

    let mut input = String::new();

    b.play_x(1, 1);
    b.print();
    b.play_from_input(Player::O);
    b.print();
    println!("Done!")
}

struct Board {
    b: [[Option<Player>; 3]; 3],
    turn: Player,
}

impl Board {
    fn new() -> Board {
        Board {
            b: [[None; 3]; 3],
            turn: Player::X,
        }
    }

    fn finnished(&self) -> bool {
        for i in 0..3 {
            if self.b[i][0].is_none() | self.b[i][1].is_none() | self.b[i][2].is_none() {
                continue;
            }
        }
        true
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

    fn play_from_input(&mut self, turn: Player) {
        loop {
            println!("Enter input: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut parts = input.split_whitespace().map(|s| s.parse::<usize>());
            let (i, j) = match (parts.next(), parts.next()) {
                (Some(Ok(a)), Some(Ok(b))) => (a, b),
                _ => {
                    println!("ERROR: input must be like; i j");
                    continue;
                }
            };
            if self.b[i][j].is_none() {
                self.b[i][j] = Some(turn);
                break;
            }
            println!("ERROR: space already taken")
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
