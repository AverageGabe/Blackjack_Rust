use rand::seq::SliceRandom;
use std::io;
use std::thread;

fn main() {
    let mut stand = false;
    let mut player_hand: i16 = 0;
    let mut dealer_hand: i16 = 0;
    let mut cards = vec![
        2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11, 2,
        3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11,
    ];
    cards.shuffle(&mut rand::thread_rng());

    println!("Welcome to shitty blackjack **In Rust**");

    'outer: loop {
        if player_hand == 21 {
            println!("Blackjack!, You win");
            break;
        } else if player_hand > 21 {
            println!("Bust!, You lose");
            break;
        }
        if !stand {
            let mut input = String::new();
            println!("Your hand is {}", player_hand);

            io::stdin()
                .read_line(&mut input)
                .expect("Please dont be retarded");

            if input.trim() == "Hit" || input.trim() == "hit" {
                let choice = cards[cards.len() - 1] as i16;
                player_hand += choice;
                cards.pop();
            } else if input.trim() == "Stand" || input.trim() == "stand" {
                stand = true;
            }
        } else {
            while dealer_hand <= 16 {
                let dchoice = cards[cards.len() - 1] as i16;
                dealer_hand += dchoice;
                println!("Dealer Hand is: {}", dealer_hand);
                thread::sleep_ms(500);

                if dealer_hand == 21 {
                    println!("Dealer Blackjack, You lose");
                    break 'outer;
                } else if dealer_hand > 21 {
                    println!("Dealer Bust, You win");
                    break 'outer;
                }
            }
            if player_hand > dealer_hand {
                println!("You win");
                break;
            } else {
                println!("Dealer wins");
                break;
            }
        }
    }
}
