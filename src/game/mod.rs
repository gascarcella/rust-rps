pub mod player;
pub mod symbol;
pub mod winner;

pub use player::*;
pub use symbol::*;
pub use winner::*;

#[derive(Debug)]
pub struct Game {
    pub player_1: Player,
    pub player_2: Player,
    pub max_rounds: i8,
    pub round: i8,
    pub ties: i8,
}

impl Game {
    pub fn new(player_1_type: PlayerType, player_2_type: PlayerType, max_rounds: i8) -> Self {
        Self {
            player_1: Player::new(player_1_type),
            player_2: Player::new(player_2_type),
            max_rounds,
            round: 0,
            ties: 0,
        }
    }

    pub fn start(&mut self) {
        println!(
            "Starting game between {:?} and {:?} for {} rounds",
            self.player_1.player_type,
            self.player_2.player_type,
            self.max_rounds,
        );
        self.next_round();
    }

    pub fn end(&self) {
        println!("Game Ended");
        match self.game_winner() {
            Winner::PlayerA => println!("Player 1 Won the Game"),
            Winner::PlayerB => println!("Player 2 Won the Game"),
            Winner::Tie => println!("Game TIE"),
        }
        self.print_status()
    }

    pub fn print_status(&self) {
        println!("Player 1 Score {}", self.player_1.score);
        println!("Player 2 Score {}", self.player_2.score);
        println!("Ties {}", self.ties);
    }

    fn next_round(&mut self) {
        self.round = self.round + 1;
        let player1_choice = self.player_1.get_input();
        let player2_choice = self.player_2.get_input();
        let winner = self.get_winner(player1_choice, player2_choice);
        match winner {
            Winner::PlayerA => {
                self.player_1.scored();
                println!("Player 1 Won with {:?} against {:?}", player1_choice, player2_choice);
            },
            Winner::PlayerB => {
                self.player_2.scored();
                println!("Player 2 Won with {:?} against {:?}", player2_choice, player1_choice);
            },
            Winner::Tie => {
                self.ties += 1;
                println!("Round TIE");
            },
        }
        return if !self.finished() {
            self.print_status();
            self.next_round();
        } else {
            self.end();
        }
    }

    fn get_winner(&self, player1_choice: Symbol, player2_choice: Symbol) -> Winner {
        if (player1_choice as i8 == player2_choice as i8 + 1) | ((player1_choice == Symbol::Rock) & (player2_choice == Symbol::Scissors)) {
            Winner::PlayerA
        }
        else if player1_choice == player2_choice {
            Winner::Tie
        } else{
            Winner::PlayerB
        }
    }

    fn game_winner(&self) -> Winner {
        return if self.player_1.score > self.player_2.score {
            Winner::PlayerA
        } else if self.player_2.score > self.player_1.score {
            Winner::PlayerB
        } else {
            Winner::Tie
        }
    }

    pub fn finished(&mut self) -> bool {
        self.round >= self.max_rounds
    }
}