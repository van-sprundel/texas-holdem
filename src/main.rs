use std::fmt::{Debug, Display, Formatter};
use rand::prelude::SliceRandom;
use rand::thread_rng;

fn main() {
    let player1 = Player::new("Player");
    let player2 = Player::new("CPU1");
    let player3 = Player::new("CPU2");
    let player4 = Player::new("CPU3");
    let mut b = Board::new(vec![player1, player2, player3, player4]);

    let stdin = std::io::stdin();
    loop { // game loop
        for (i, player) in b.players.iter_mut().enumerate() {
            println!("It's {}'s turn!", player.name);
            println!("{}'s cards: {:?}", player.name, player.cards);
            loop {
                println!("What will be the move?");
                let mut buffer = String::new();
                stdin.read_line(&mut buffer);
                match &*buffer.trim().to_lowercase() {
                    "raise" | "r" => {
                        println!("Raised!");
                        b.players_done = [false; 4];
                        b.players_done[i] = true;
                        break;
                    }
                    "c" | "call" => {
                        println!("Called!");
                        b.players_done[i] = true;
                        break;
                    }
                    "q" | "quit" | "exit" | "e" => panic!("Quitting..."),
                    _ => {}
                }
            }
            if b.players_done.iter().filter(|x| **x).count() == b.players_done.len() {
                // println!("Everyone checked or called when a raise happened.");
                b.players_done = [false; 4];
                b.round += 1;
                match b.round {
                    1 => {
                        println!("Show the first 3 cards.");
                        println!("Cards in middle: {:?}", b.middle_cards);
                    }
                    2 => {
                        println!("Show the 4th card.");
                    }
                    3 => {
                        println!("Show the 5th card.");
                        println!("Showtime, baby!");
                        // showdown
                    }
                    _ => {}
                }
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
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
    balance: i32,
    bet: i32,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_lowercase().to_string(),
            cards: Vec::with_capacity(2),
            balance: 10_000,
            bet: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    round: u8,
    players: Vec<Player>,
    players_done: [bool; 4],
    deck: Deck,
    middle_cards: Vec<Card>,
}

impl Board {
    pub fn new(p: Vec<Player>) -> Self {
        let mut deck = Deck::create_deck();
        let mut players = Vec::with_capacity(4);
        players.extend(p);
        deck.shuffle_deck();
        let mut board = Self {
            round: 0,
            players: players.clone(),
            players_done: [false; 4],
            deck,
            middle_cards: Vec::with_capacity(5),
        };

        for x in players.iter() {
            for _ in 0..2 {
                board.draw_player(&x.name);
            }
        }
        for _ in 0..5 {
            board.draw_middle();
        }
        board
    }
    pub fn draw_player(&mut self, name: &str) {
        let player = self.players.iter_mut().find(|x| x.name == name.to_lowercase()).expect("Couldn't find player");
        player.cards.push(self.deck.draw());
    }
    pub fn draw_middle(&mut self) {
        let card = self.deck.draw();
        self.middle_cards.push(card);
    }
    pub fn get_cards(&self, name: &str) -> &Vec<Card> {
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
