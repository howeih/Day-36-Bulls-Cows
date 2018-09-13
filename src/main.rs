extern crate rand;
use rand::seq::sample_iter;
use rand::thread_rng;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
struct Player {
    secret: HashSet<i32>,
    wins: bool,
}

struct Game {
    player1: Player,
    player2: Player,
    turn: usize,
}

impl Player {
    fn new() -> Player {
        let mut rng = thread_rng();
        let sample = sample_iter(&mut rng, 1..10, 4).unwrap();
        let secret: HashSet<i32> = HashSet::from_iter(sample.into_iter());
        Player {
            secret: secret,
            wins: false,
        }
    }
    fn guess(&self) -> HashSet<i32> {
        let mut rng = thread_rng();
        let sample = sample_iter(&mut rng, 1..10, 4).unwrap();
        return HashSet::from_iter(sample.into_iter());
    }

    fn score(&self, guess: HashSet<i32>) -> (i32, i32) {
        let mut bulls = 0i32;
        for (i, j) in self.secret.iter().zip(guess.iter()) {
            if i == j {
                bulls += 1;
            }
        }
        let cows = self.secret.intersection(&guess).count() as i32 - bulls;
        (bulls, cows)
    }
}

impl Game {
    fn new(player1: Player, player2: Player) -> Game {
        Game {
            player1: player1,
            player2: player2,
            turn: 0,
        }
    }

    fn start(&mut self) {
        while !self.player1.wins && !self.player2.wins {
            if self.turn % 2 == 0 {
                let player1_guess = self.player1.guess();
                let (bulls, cows) = self.player2.score(player1_guess);
                if bulls < 4 {
                    println!("bulls: {}, cows: {}", bulls, cows);
                } else {
                    self.player1.wins = true;
                    print!("player1 wins");
                }
            } else {
                let player2_guess = self.player2.guess();
                let (bulls, cows) = self.player1.score(player2_guess);
                if bulls < 4 {
                    println!("bulls: {}, cows: {}", bulls, cows);
                } else {
                    self.player1.wins = true;
                    print!("player2 wins");
                }
            }
            self.turn += 1;
        }
    }
}

fn main() {
    let player = Player::new();
    let player2 = Player::new();
    println!("{:?} {:?}", player, player2);
    let mut game = Game::new(player, player2);
    game.start();
}
