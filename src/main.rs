mod card;
mod deck;
mod hand;

use crate::{deck::Deck, hand::Hand};
use colored::Colorize;
use std::{io::stdin, process::exit};
use std::{thread, time};

fn show_menu() {
    println!("------------------------------");
    println!(
        "{} {} {}",
        "1. Check".bold(),
        "your".bold().blue(),
        "hand".bold()
    );
    println!(
        "{} {} {}",
        "2. Check".bold(),
        "dealer".bold().cyan(),
        "hand".bold()
    );
    println!("{}", "3. Hit".bold());
    println!("{}", "4. Stand".bold());
    println!("{}", "5. Print this menu".bold());
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
            player.hit(deck);
            true
        }
        "4" => {
            println!("You have decided to {}.", "stand".bold());
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
    println!("{}", "\nWelcome to Blackjack!".bold());
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
        let player_over = player.check_blackjack_player();
        if !player_over {
            let mut option = String::new();
            let _ = stdin().read_line(&mut option);
            // TODO solve this error
            let flag = check_option(option, &mut player, &dealer, &mut deck);
            if !flag {
                break;
            }
        } else {
            println!("{} cards where: ", "Your".bold().blue());
            player.check_cards();
            println!("{} cards where: ", "Dealer".bold().cyan());
            dealer.check_cards();
            println!("{} \nThanks for playing.", "The game is over.".bold());
            exit(1);
        }
    }
    // dealer loop
    loop {
        let dealer_over = dealer.check_blackjack_dealer();
        thread::sleep(time::Duration::from_secs(3));
        if !dealer_over {
            dealer.hit(&mut deck);
        } else {
            println!("{} cards where: ", "Your".bold().blue());
            player.check_cards();
            println!("{} cards where: ", "Dealer".bold().cyan());
            dealer.check_cards();
            println!("{} \nThanks for playing.", "The game is over.".bold());
            exit(1);
        }
    }
}
