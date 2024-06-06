use core::fmt;

pub struct Card {
    pub suit: String,
    pub value: String,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Suit: {}, Value: {}", self.suit, self.value)
    }
}

impl Card {
    fn print_a() {
        println!("+-----+");
        println!("| A   |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    fn print_2() {
        println!("+-----+");
        println!("| 2   |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    fn print_3() {
        println!("+-----+");
        println!("| 3   |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    fn print_4() {
        println!("+-----+");
        println!("| 4   |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    fn print_5() {
        println!("+-----+");
        println!("| 5   |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    fn print_6() {
        println!("+-----+");
        println!("| 6   |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    fn print_7() {
        println!("+-----+");
        println!("| 7   |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    fn print_8() {
        println!("+-----+");
        println!("| 8   |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    fn print_9() {
        println!("+-----+");
        println!("| 9   |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    fn print_10() {
        println!("+-----+");
        println!("| 10  |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    fn print_j() {
        println!("+-----+");
        println!("| J   |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    fn print_q() {
        println!("+-----+");
        println!("| Q   |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    fn print_k() {
        println!("+-----+");
        println!("| K   |");
        println!("|     |");
        println!("|     |");
        println!("|     |");
        println!("+-----+");
    }
    pub fn print_facedown() {
        println!("+-----+");
        println!("| *** |");
        println!("| *** |");
        println!("| *** |");
        println!("| *** |");
        println!("+-----+");
    }
    pub fn print_exact_card(&self) {
        match self.value.as_str() {
            "A" => Card::print_a(),
            "2" => Card::print_2(),
            "3" => Card::print_3(),
            "4" => Card::print_4(),
            "5" => Card::print_5(),
            "6" => Card::print_6(),
            "7" => Card::print_7(),
            "8" => Card::print_8(),
            "9" => Card::print_9(),
            "10" => Card::print_10(),
            "J" => Card::print_j(),
            "Q" => Card::print_q(),
            "K" => Card::print_k(),
            _ => {}
        }
        println!("{}", self.suit);
    }
}
