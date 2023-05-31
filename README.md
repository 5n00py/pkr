# pkr

"pkr" is a Rust library that provides basic components to model and evaluate a 
poker game. It includes basic entities like Card, Rank, Suit, Deck and Hand
along with functionalities to shuffle and deal cards. Furthermore it provides 
a set of tools for creating and evaluating poker hands. The library has been 
built with efficiency in mind, with performance critical components written in 
a way to minimize unnecessary computation.

## Installation

You can download and use this library in your Rust project by adding the 
following to your `Cargo.toml`:

```toml
[dependencies]
pkr = "0.1.1"
```

## Features

- Define poker hands with standard string notation (i.e. "As Ks Qs Js Ts").
- Evaluate poker hands according to the standard rules of poker, including 
support for hand types such as Straight Flush, Four of a Kind, Full House, 
Flush, Straight, Three of a Kind, Two Pair, One Pair and High Card.
- Calculate the numerical score of a poker hand for comparison.
- Ability to evaluate poker hands consisting of 2 up to 9 cards.
- Create a deck consisting of 52 cards and shuffle the deck.

## Examples

Create a Hand struct:

```Rust
use pkr::hand::Hand;
hand1 = Hand::new_from_str("Ts Js Qs Ks As").unwrap();
```
This creates a Royal Flush hand.

Once you have a Hand you can evaluate its score:
```Rust
let score1 = hand1.get_score();
```

This returns an u32 score representing the value of the hand according to the 
poker rules and makes it comparable:

```Rust
hand2 = Hand::new_from_str("As Ad Ah Ac Ks").unwrap();
score2 = hand2.get_score();
assert!(score1 > score2);
```

Please note that this library does not handle game progression (yet) - it 
simply provides a way to evaluate poker hands.

## License

This project is licensed under the GNU General Public License v3.0.
