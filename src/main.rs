use std::fmt::{Debug, Display, Formatter};
use std::io::Read;
use rand::prelude::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut player = Player::new("Ramon");
    let mut b = Board::new(vec![player]);

    let mut stdin = std::io::stdin();

    b.draw_card("ramon");
    b.draw_card("ramon");
    loop {
        let mut buffer = String::new();
        println!("Your cards: {:?}",b.get_cards("ramon"));
        println!("What will be your move?");
        stdin.read_line(&mut buffer);
        match &*buffer.trim().to_lowercase() {
            "raise" | "r" => {
                println!("You raised!");
            }
            "c" | "call" => {
                println!("You called!");
            }
            "q" | "quit" | "exit" | "e" => break,
            _ => {}
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jacks,
    Queen,
    King,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Suit {
    Spades,
    Diamonds,
    Hearts,
    Clubs,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Black,
}

#[derive(Clone, Copy)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", match self.rank {
            Rank::Ace => "Ace",
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jacks => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King"
        }, match self.suit {
            Suit::Spades => "Spades",
            Suit::Diamonds => "Diamonds",
            Suit::Hearts => "Hearts",
            Suit::Clubs => "Clubs"
        })
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", match self.rank {
            Rank::Ace => "Ace",
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jacks => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King"
        }, match self.suit {
            Suit::Spades => "Spades",
            Suit::Diamonds => "Diamonds",
            Suit::Hearts => "Hearts",
            Suit::Clubs => "Clubs"
        })
    }
}

#[derive(Debug, Clone)]
struct Player {
    name: String,
    cards: Vec<Card>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_lowercase().to_string(),
            cards: Vec::with_capacity(2),
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    players: Vec<Player>,
    deck: Deck,
    middle_cards: [Option<Card>; 5],
}

impl Board {
    pub fn new(p: Vec<Player>) -> Self {
        let mut players = Vec::with_capacity(4);
        players.extend(p);
        let mut deck = Deck::create_deck();
        deck.shuffle_deck();
        Self {
            players,
            deck,
            middle_cards: [None; 5],
        }
    }
    pub fn draw_card(&mut self, name: &str) {
        let player = self.players.iter_mut().find(|x| x.name == name.to_lowercase()).expect("Couldn't find player");
        player.cards.push(self.deck.draw());
    }
    pub fn get_cards(&self, name:&str)->&Vec<Card> {
        &self.players.iter().find(|x| x.name == name.to_lowercase()).expect("Couldn't find player").cards
    }
}

impl Card {
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        Self {
            suit,
            rank,
        }
    }
    pub const fn get_color(&self) -> Color {
        match self.suit {
            Suit::Spades => Color::Black,
            Suit::Diamonds => Color::Red,
            Suit::Hearts => Color::Red,
            Suit::Clubs => Color::Black,
        }
    }
    pub const fn get_value(&self) -> i32 {
        match self.rank {
            Rank::Ace => 11,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jacks => 12,
            Rank::Queen => 13,
            Rank::King => 14
        }
    }
}

#[derive(Debug, Clone)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn create_deck() -> Self {
        Self {
            cards: [
                Card::new(Rank::Two, Suit::Clubs),
                Card::new(Rank::Ace, Suit::Clubs),
                Card::new(Rank::Three, Suit::Clubs),
                Card::new(Rank::Four, Suit::Clubs),
                Card::new(Rank::Five, Suit::Clubs),
                Card::new(Rank::Six, Suit::Clubs),
                Card::new(Rank::Seven, Suit::Clubs),
                Card::new(Rank::Eight, Suit::Clubs),
                Card::new(Rank::Nine, Suit::Clubs),
                Card::new(Rank::Ten, Suit::Clubs),
                Card::new(Rank::Jacks, Suit::Clubs),
                Card::new(Rank::Queen, Suit::Clubs),
                Card::new(Rank::King, Suit::Clubs),
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::Two, Suit::Spades),
                Card::new(Rank::Three, Suit::Spades),
                Card::new(Rank::Four, Suit::Spades),
                Card::new(Rank::Five, Suit::Spades),
                Card::new(Rank::Six, Suit::Spades),
                Card::new(Rank::Seven, Suit::Spades),
                Card::new(Rank::Eight, Suit::Spades),
                Card::new(Rank::Nine, Suit::Spades),
                Card::new(Rank::Ten, Suit::Spades),
                Card::new(Rank::Jacks, Suit::Spades),
                Card::new(Rank::Queen, Suit::Spades),
                Card::new(Rank::King, Suit::Spades),
                Card::new(Rank::Ace, Suit::Hearts),
                Card::new(Rank::Two, Suit::Hearts),
                Card::new(Rank::Three, Suit::Hearts),
                Card::new(Rank::Four, Suit::Hearts),
                Card::new(Rank::Five, Suit::Hearts),
                Card::new(Rank::Six, Suit::Hearts),
                Card::new(Rank::Seven, Suit::Hearts),
                Card::new(Rank::Eight, Suit::Hearts),
                Card::new(Rank::Nine, Suit::Hearts),
                Card::new(Rank::Ten, Suit::Hearts),
                Card::new(Rank::Jacks, Suit::Hearts),
                Card::new(Rank::Queen, Suit::Hearts),
                Card::new(Rank::King, Suit::Hearts),
                Card::new(Rank::Ace, Suit::Diamonds),
                Card::new(Rank::Two, Suit::Diamonds),
                Card::new(Rank::Three, Suit::Diamonds),
                Card::new(Rank::Four, Suit::Diamonds),
                Card::new(Rank::Five, Suit::Diamonds),
                Card::new(Rank::Six, Suit::Diamonds),
                Card::new(Rank::Seven, Suit::Diamonds),
                Card::new(Rank::Eight, Suit::Diamonds),
                Card::new(Rank::Nine, Suit::Diamonds),
                Card::new(Rank::Ten, Suit::Diamonds),
                Card::new(Rank::Jacks, Suit::Diamonds),
                Card::new(Rank::Queen, Suit::Diamonds),
                Card::new(Rank::King, Suit::Diamonds),
            ].to_vec()
        }
    }
    pub fn shuffle_deck(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
    pub fn draw(&mut self) -> Card {
        self.cards.pop().expect("Deck is empty")
    }
}
