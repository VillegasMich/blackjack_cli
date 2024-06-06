mod card;
mod deck;
mod hand;

use crate::{deck::Deck, hand::Hand};
use std::{io::stdin, process::exit};

fn show_menu() {
    println!("------------------------------");
    println!("1. Check your hand");
    println!("2. Check dealer hand");
    println!("3. Hit");
    println!("4. Stand");
    println!("5. Print this menu");
    println!("------------------------------");
}
fn check_option(option: String, player: &mut Hand, dealer: &Hand, deck: &mut Deck) -> bool {
    match option.as_str().trim() {
        "1" => {
            player.check_cards();
            true
        }
        "2" => {
            dealer.check_dealer_cards();
            true
        }
        "3" => {
            player.give_card(deck);
            true
        }
        "4" => {
            println!("You have decided to stand.");
            false
        }
        "5" => {
            show_menu();
            true
        }
        _ => {
            println!("You need to enter an option from the menu.");
            true
        }
    }
}

fn main() {
    let mut player_over: bool;
    println!("\nWelcome to Blackjack!");
    let mut deck = Deck::new();
    let mut player = Hand::new();
    let mut dealer = Hand::new();
    player.give_card(&mut deck);
    dealer.give_card(&mut deck);
    player.give_card(&mut deck);
    dealer.give_card(&mut deck);
    show_menu();
    // player loop
    loop {
        player_over = player.check_blackjack_player();
        if !player_over {
            let mut option = String::new();
            let _ = stdin().read_line(&mut option);
            // TODO solve this error
            let flag = check_option(option, &mut player, &dealer, &mut deck);
            if !flag {
                break;
            }
        } else {
            println!("Your cards where: ");
            player.check_cards();
            println!("Dealer cards where: ");
            dealer.check_cards();
            println!("The game is over.\nThanks for playing.");
            exit(1);
        }
    }
    // dealer loop
    loop {
        let dealer_over = dealer.check_blackjack_dealer();
        if !dealer_over {
            dealer.give_card(&mut deck);
        } else {
            println!("Your cards where: ");
            player.check_cards();
            println!("Dealer cards where: ");
            dealer.check_cards();
            println!("The game is over.\nThanks for playing.");
            exit(1);
        }
    }
}
