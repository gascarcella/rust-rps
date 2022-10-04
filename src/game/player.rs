use text_io::read;

use crate::Symbol;


#[derive(Debug)]
pub enum PlayerType {
    Human,
    Computer,
}

#[derive(Debug)]
pub struct Player {
    pub player_type: PlayerType,
    pub score: i8,
}

impl Player {
    pub fn new(player_type: PlayerType) -> Self {
        Self {
            player_type,
            score: 0,
        }
    }

    pub fn scored(&mut self) {
        self.score += 1;
    }

    pub fn get_input(&self) -> Symbol {
        match self.player_type {
            PlayerType::Human => self.get_human_input(),
            PlayerType::Computer => self.get_computer_input()
        }
    }

    fn get_human_input(&self) -> Symbol {
        println!("Choose [R]ock, [P]aper, [S]cissor. Input R,P,S or 1,2,3 and press [Enter]");
        let value: String = read!("{}\n");
        match value.to_lowercase().as_str() {
            "r" | "1" => Symbol::Rock,
            "p" | "2" => Symbol::Paper,
            "s" | "3" => Symbol::Scissors,
            _ => {
                println!("Invalid Input, please try again");
                self.get_human_input()
            }
        }
    }

    fn get_computer_input(&self) -> Symbol {
        Symbol::Rock
    }
}
