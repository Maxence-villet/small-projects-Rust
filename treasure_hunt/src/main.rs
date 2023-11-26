use std::io;
use rand::Rng;
use std::{thread, time};

enum TreasureHuntState {
    Playing,
    FoundTreasure,
}


struct Player {
    position: (i32, i32),
    treasure_position: (i32, i32),
}

impl Player {
    fn move_up(&mut self) {
        self.position.1 += 1;
    }

    fn move_down(&mut self) {
        self.position.1 -= 1;
    }

    fn move_left(&mut self) {
        self.position.0 -= 1;
    }

    fn move_right(&mut self) {
        self.position.0 += 1;
    }

    fn check_treasure(&self) -> TreasureHuntState {
        if self.position == self.treasure_position {
            TreasureHuntState::FoundTreasure
        } else {
            TreasureHuntState::Playing
        }
    }
}

fn main() {

    let range_position_treasure = 7;

    let treasure_x = rand::thread_rng().gen_range(0..range_position_treasure);
    let treasure_y = rand::thread_rng().gen_range(0..range_position_treasure);

    let mut player = Player {
        position: (0,0),
        treasure_position: (treasure_x,treasure_y),
    };

    let mut state = TreasureHuntState::Playing;

    println!("+--------------------------------+");
    println!("| move : (up, down, left, right) |");
    println!("+--------------------------------+");

    println!("+----------------------+");
    println!("| range treasure : {}x{} |", range_position_treasure.to_string(), range_position_treasure.to_string());
    println!("+----------------------+");

    loop {
        

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        match input {
            "up" => player.move_up(),
            "down" => player.move_down(),
            "left" => player.move_left(),
            "right" => player.move_right(),
            _ => println!("Invalid move"),
        }   

        state = player.check_treasure();
        
        match state {
            TreasureHuntState::Playing => println!("x: {}, y: {}", player.position.0, player.position.1),
            TreasureHuntState::FoundTreasure => {println!("You found the treasure!"); break;},
            _ => panic!("Invalid state"),
        }

    }

    println!("+--------------------------+");
    println!("| © Made By Maxence-villet |");
    println!("+--------------------------+");


    thread::sleep(time::Duration::from_secs(3));
}



// Version 1.0
