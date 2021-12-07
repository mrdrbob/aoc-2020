use std::collections::VecDeque;

pub fn execute() {
    let file = std::fs::read_to_string(".\\data\\22.txt").unwrap();
    let split:Vec<&str> = file.split("\n\n").collect();
    let one = Deck::from_string(split[0]);
    let two = Deck::from_string(split[1]);

    let mut game = Game { one: one, two: two };
    let winner = game.find_winner();
    let winning_deck = match winner {
        1 => game.one,
        2 => game.two,
        _ => unimplemented!()
    };

    let score = winning_deck.score();

    println!("{} {}", winner, score);
}

struct Game {
    one: Deck,
    two: Deck
}

impl Game {
    fn find_winner(&mut self) -> usize {
        match self.iterate() {
            None => self.find_winner(),
            Some(winner) => winner
        }
    }

    fn iterate(&mut self) -> Option<usize> {
        println!("{} {}", self.one.cards.len(), self.two.cards.len());
        match self.one.take_top() {
            None => Some(2),
            Some (one_card) => {
                match self.two.take_top() {
                    None => {
                        // Put that card we popped back on top of the deck.
                        self.one.cards.push_front(one_card);
                        Some(1)
                    },
                    Some (two_card) => {
                        if one_card > two_card {
                            self.one.cards.push_back(one_card);
                            self.one.cards.push_back(two_card);
                            None
                        } else if two_card > one_card {
                            self.two.cards.push_back(two_card);
                            self.two.cards.push_back(one_card);
                            None
                        } else {
                            unimplemented!()
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Deck {
    cards:VecDeque<usize>
}

impl Deck {
    fn from_string(raw:&str) -> Deck {
        let t:VecDeque<usize> = raw.lines().skip(1).map(|line| line.parse().unwrap()).collect();
        Deck { cards: t }
    }

    fn take_top(&mut self) -> Option<usize> {
        self.cards.pop_front()
    }

    fn score(&self) -> usize {
        let length = self.cards.len();
        (0..length).fold(0usize, |acc, idx| { ((length - idx) * self.cards[idx]) + acc })
    }
}