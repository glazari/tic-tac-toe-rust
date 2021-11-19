use std::io;

fn main() {
    let mut b = Board::new();
    b.play();
}

struct Board {
    b: [[Option<Player>; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board { b: [[None; 3]; 3] }
    }

    fn play(&mut self) {
        let mut turn = Player::X;

        while self.winner().is_none() {
            self.print();
            self.play_from_input(turn);
            turn = match turn {
                Player::X => Player::O,
                Player::O => Player::X,
            }
        }
        self.print();
        println!("winner: {:?}", self.winner().unwrap());
        println!("Done!")
    }

    fn winner(&self) -> Option<Player> {
        for i in 1..3 {
            if self.b[i].iter().all(|x| *x == Some(Player::X)) {
                return Some(Player::X);
            }
            if self.b[i].iter().all(|x| *x == Some(Player::O)) {
                return Some(Player::O);
            }
            if self.b.iter().all(|x| x[i] == Some(Player::X)) {
                return Some(Player::X);
            }
            if self.b.iter().all(|x| x[i] == Some(Player::O)) {
                return Some(Player::O);
            }
        }

        None
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
            println!("Player's {} turn: ", turn);
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
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
