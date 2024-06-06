# Rust Blackjack Cli

## Game Logic:

- Card Struct: Define a Card struct with fields for suit (e.g., Hearts, Spades) and value (e.g., Ace, 2-10, Jack, Queen, King).
- Deck Struct: Define a Deck struct that holds a vector of Cards. Implement methods for shuffling the deck using the rand crate and dealing cards.
- Hand Struct: Define a Hand struct to represent the player's and dealer's cards. Implement methods for adding cards, calculating the total value (considering Ace as 1 or 11), and checking for bust (total exceeding 21).

## Game Loop: Implement the core game loop:

1. Initialize a deck, player hand, and dealer hand.
1. Deal initial cards for player and dealer.
1. Ask the player for actions (hit or stand) in a loop.
1. Deal cards to the player based on their choice.
1. Handle dealer's turn by automatically hitting until reaching a certain value (e.g., 17).
1. Evaluate who wins (player, dealer, or tie) based on hand values.

## TODO

- [ ] Make the **A** card has value 1 and 11;
- [ ] Add delay time between dealer actions;
- [ ] Print information with colors;
