mod cards;
mod radlands;

use radlands::camps::CampType;
use radlands::people::PersonType;
use radlands::*;

use crate::radlands::controllers::{
    human::HumanController, random::RandomController, PlayerController,
};

fn main() {
    println!("AutoRad, version {}\n", env!("CARGO_PKG_VERSION"));

    let camp_types = camps::get_camp_types();
    let person_types = people::get_person_types();

    let do_random = std::env::args().any(|arg| arg == "--random");

    let num_games = if do_random { 10_000 } else { 1 };
    for _ in 0..num_games {
        do_game(&camp_types, &person_types, do_random);
    }
}

fn do_game(camp_types: &[CampType], person_types: &[PersonType], random: bool) {
    let p1: &dyn PlayerController;
    let p2: &dyn PlayerController;
    if random {
        p1 = &RandomController;
        p2 = &RandomController;
    } else {
        p1 = &HumanController { label: "Human 1" };
        p2 = &HumanController { label: "Human 2" };
    }
    // let hc1 = RandomController;
    // let hc2 = RandomController;
    let mut game_state = GameState::new(camp_types, person_types);

    for turn_num in 1.. {
        println!("\nTurn {}\n", turn_num);
        if let Err(result) = game_state.do_turn(p1, p2, turn_num == 1) {
            println!(
                "\nGame ended; {}",
                match result {
                    GameResult::P1Wins => "player 1 wins!",
                    GameResult::P2Wins => "player 2 wins!",
                    GameResult::Tie => "tie!",
                }
            );
            break;
        }
    }

    println!("\nFinal state:\n{}", game_state);
}
