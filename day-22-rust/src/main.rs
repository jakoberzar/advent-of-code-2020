use std::hash::Hash;
use std::{
    collections::{HashSet, VecDeque},
    iter::FromIterator,
};

#[allow(dead_code)]
const INPUT: &str = include_str!("./../../inputs/day-22.txt");
#[allow(dead_code)]
const SIMPLE_INPUT: &str = include_str!("./../../inputs/simple/day-22.txt");

const MAX_CARD_VALUE: usize = 50;
const CARD_PER_U64: usize = 11; // log_51(2^64) = 11
const REQUIRED_U64_BYTES: usize = 5; // MAX_CARD_VALUE / CARD_PER_U64
const VALUE_MULTIPLIER: u64 = (MAX_CARD_VALUE as u64) + 1;

fn main() {
    let players = parse_input(INPUT);

    println!(
        "Score of combat's winner is {}",
        star1(&players.0, &players.1)
    );
    println!(
        "Score of recursive combat's winner is {}",
        star2(&players.0, &players.1)
    );
}

fn parse_input(input: &str) -> (VecDeque<u8>, VecDeque<u8>) {
    let mut players = input.trim().split("\n\n");
    let player1 = parse_player(players.next().unwrap());
    let player2 = parse_player(players.next().unwrap());

    // Check max
    let actual_max_value = player1.iter().chain(player2.iter()).max().unwrap();
    assert!(*actual_max_value as usize <= MAX_CARD_VALUE);

    (player1, player2)
}

fn parse_player(input: &str) -> VecDeque<u8> {
    input
        .trim()
        .lines()
        .skip(1)
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn star1(player1: &VecDeque<u8>, player2: &VecDeque<u8>) -> usize {
    let mut player1 = player1.clone();
    let mut player2 = player2.clone();

    // Play game
    while !player1.is_empty() && !player2.is_empty() {
        let play1 = player1.pop_front().unwrap();
        let play2 = player2.pop_front().unwrap();
        let winner = if play1 > play2 {
            &mut player1
        } else {
            &mut player2
        };
        winner.push_back(play1.max(play2));
        winner.push_back(play1.min(play2));
    }

    // Get the winner
    let winner = if player1.is_empty() {
        &player2
    } else {
        &player1
    };

    // Calculate winner's score
    calculate_score(winner)
}

fn calculate_score(deck: &VecDeque<u8>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(idx, value)| (idx + 1) * (*value as usize))
        .sum()
}

fn star2(player1: &VecDeque<u8>, player2: &VecDeque<u8>) -> usize {
    let mut recursive_combat = RecursiveCombat::new(player1.clone(), player2.clone());
    let winner = recursive_combat.play();
    calculate_score(recursive_combat.get_players_deck(winner))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    Player1,
    Player2,
}

impl Player {
    fn opponent(self) -> Self {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DeckState {
    card_sums: [u64; REQUIRED_U64_BYTES], // Assertion checked in parse_input()
}

impl DeckState {
    fn new(deck1: &VecDeque<u8>, deck2: &VecDeque<u8>) -> Self {
        let mut card_sums = [0; REQUIRED_U64_BYTES];

        // Insert deck1
        let mut idx = 0;
        for element in deck1.iter() {
            let arr_idx = idx / CARD_PER_U64;
            card_sums[arr_idx] = (card_sums[arr_idx] * VALUE_MULTIPLIER) + (*element as u64);
            idx += 1;
        }

        // Add separator
        card_sums[idx / CARD_PER_U64] *= VALUE_MULTIPLIER;
        idx += 1;

        // Insert deck2
        for element in deck2.iter() {
            let arr_idx = idx / CARD_PER_U64;
            card_sums[arr_idx] = (card_sums[arr_idx] * VALUE_MULTIPLIER) + (*element as u64);
            idx += 1;
        }

        // More elegant, but slower solution:
        // let separator = Some(&0u8);
        // let iterator = deck1
        //     .iter()
        //     .chain(separator.into_iter())
        //     .chain(deck2.iter());
        // for (idx, element) in iterator.enumerate() {
        //     let arr_idx = idx / CARD_PER_U64;
        //     card_sums[arr_idx] = (card_sums[arr_idx] * VALUE_MULTIPLIER) + (*element as u64);
        // }
        DeckState { card_sums }
    }
}

impl Hash for DeckState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let hash = self.card_sums[0] ^ self.card_sums[2];
        state.write_u64(hash);
    }
}

struct RecursiveCombat {
    deck1: VecDeque<u8>,
    deck2: VecDeque<u8>,
    round_cache: HashSet<DeckState>,
}

impl RecursiveCombat {
    fn new(deck1: VecDeque<u8>, deck2: VecDeque<u8>) -> Self {
        RecursiveCombat {
            deck1,
            deck2,
            round_cache: HashSet::new(),
        }
    }

    fn check_cache_and_insert(&mut self) -> bool {
        !self
            .round_cache
            .insert(DeckState::new(&self.deck1, &self.deck2))
    }

    // Play the game and return the winner
    fn play(&mut self) -> Player {
        while !self.one_deck_empty() {
            if self.check_cache_and_insert() {
                return Player::Player1;
            }

            let draw1 = self.deck1.pop_front().unwrap();
            let draw2 = self.deck2.pop_front().unwrap();

            let winner: Player = if Self::player_has_enough_cards(&self.deck1, draw1)
                && Self::player_has_enough_cards(&self.deck2, draw2)
            {
                // Start a new round of recursive combat!
                let copied_deck1: VecDeque<u8> =
                    VecDeque::from_iter(self.deck1.iter().take(draw1 as usize).copied());
                let copied_deck2: VecDeque<u8> =
                    VecDeque::from_iter(self.deck2.iter().take(draw2 as usize).copied());
                let mut recursive_round = RecursiveCombat::new(copied_deck1, copied_deck2);

                let winner = recursive_round.play();
                winner
            } else {
                // Play with normal rules
                if draw1 > draw2 {
                    Player::Player1
                } else {
                    Player::Player2
                }
            };
            let winning_deck = self.get_players_deck_mut(winner);
            winning_deck.push_back(Self::players_card(winner, draw1, draw2));
            winning_deck.push_back(Self::players_card(winner.opponent(), draw1, draw2));
        }

        self.get_deck_winner()
    }

    fn player_has_enough_cards(deck: &VecDeque<u8>, amount: u8) -> bool {
        deck.len() >= (amount as usize)
    }

    fn players_card(player: Player, card1: u8, card2: u8) -> u8 {
        match player {
            Player::Player1 => card1,
            Player::Player2 => card2,
        }
    }

    fn one_deck_empty(&self) -> bool {
        self.deck1.is_empty() || self.deck2.is_empty()
    }

    fn get_players_deck(&self, player: Player) -> &VecDeque<u8> {
        match player {
            Player::Player1 => &self.deck1,
            Player::Player2 => &self.deck2,
        }
    }

    fn get_players_deck_mut(&mut self, player: Player) -> &mut VecDeque<u8> {
        match player {
            Player::Player1 => &mut self.deck1,
            Player::Player2 => &mut self.deck2,
        }
    }

    fn get_deck_winner(&self) -> Player {
        assert!(self.one_deck_empty());
        if self.deck1.is_empty() {
            Player::Player2
        } else {
            Player::Player1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_star1() {
        let (player1, player2) = parse_input(SIMPLE_INPUT);
        assert_eq!(star1(&player1, &player2), 306);
    }

    #[test]
    fn full_star1() {
        let (player1, player2) = parse_input(INPUT);
        assert_eq!(star1(&player1, &player2), 32472);
    }

    #[test]
    fn simple_star2() {
        let (player1, player2) = parse_input(SIMPLE_INPUT);
        assert_eq!(star2(&player1, &player2), 291);
    }

    #[test]
    fn full_star2() {
        let (player1, player2) = parse_input(INPUT);
        assert_eq!(star2(&player1, &player2), 36463);
    }
}
