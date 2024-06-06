use colored::Colorize;
use rand::Rng;

use crate::{card::Card, deck::Deck};

pub struct Hand {
    pub hand: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { hand: Vec::new() }
    }
    pub fn give_card(&mut self, deck: &mut Deck) {
        let rand_card = rand::thread_rng().gen_range(0..deck.cards.len() - 1);
        let card = deck.cards.remove(rand_card);
        self.hand.push(card);
    }
    pub fn check_cards(&self) {
        println!("------------------------------");
        for i in 0..self.hand.len() {
            // println!("- {}", self.hand.get(i).unwrap());
            self.hand.get(i).unwrap().print_exact_card();
        }
        println!("------------------------------");
    }
    pub fn check_dealer_cards(&self) {
        println!("------------------------------");
        println!("The {} up cards are: ", "dealer".bold().cyan());
        // println!("- Card face down");
        Card::print_facedown();
        for i in 1..self.hand.len() {
            // println!("- {}", self.hand.get(i).unwrap());
            self.hand.get(i).unwrap().print_exact_card();
        }
        println!("------------------------------");
    }
    pub fn check_blackjack_player(&self) -> bool {
        let mut sum = 0;
        for i in 0..self.hand.len() {
            let value = self.hand.get(i).unwrap().value.as_str();
            match value {
                "A" => sum += 1,
                "2" => sum += 2,
                "3" => sum += 3,
                "4" => sum += 4,
                "5" => sum += 5,
                "6" => sum += 6,
                "7" => sum += 7,
                "8" => sum += 8,
                "9" => sum += 9,
                _ => sum += 10,
            }
        }
        if sum == 21 {
            println!("------------------------------");
            println!("Congratulations you have a blackjack!");
            println!("{}.", "The player wins".bold().green());
            println!("------------------------------");
            true
        } else if sum > 21 {
            println!("------------------------------");
            println!("You have more than 21, I'm sorry you lose.");
            println!("{}.", "The player lose".bold().red());
            println!("------------------------------");
            true
        } else {
            println!("------------------------------");
            println!(
                "You have a total of {}. The game is running.",
                sum.to_string().bold().yellow()
            );
            println!("------------------------------");
            false
        }
    }
    pub fn check_blackjack_dealer(&self) -> bool {
        let mut sum = 0;
        for i in 0..self.hand.len() {
            let value = self.hand.get(i).unwrap().value.as_str();
            match value {
                "A" => sum += 1,
                "2" => sum += 2,
                "3" => sum += 3,
                "4" => sum += 4,
                "5" => sum += 5,
                "6" => sum += 6,
                "7" => sum += 7,
                "8" => sum += 8,
                "9" => sum += 9,
                _ => sum += 10,
            }
        }
        if sum == 21 {
            println!("------------------------------");
            println!("{}.", "The dealer has a blackjack, he won".bold().red());
            println!("------------------------------");
            true
        } else if sum > 21 {
            println!("------------------------------");
            println!(
                "{}.",
                "The dealer has more than 21, You are the winner"
                    .bold()
                    .green()
            );
            println!("------------------------------");
            true
        } else {
            println!("------------------------------");
            println!(
                "The dealer has a total of {}. The game is running.",
                sum.to_string().bold().yellow()
            );
            println!("------------------------------");
            false
        }
    }
}
