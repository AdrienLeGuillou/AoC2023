use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Part 1
    let mut hands: Vec<Hand> = input.lines()
                                .map(Hand::parse_string)
                                .collect();

    hands.sort();
    hands.reverse();

    let mut winnings = 0;
    for i in 0..hands.len() {
        winnings += (i as i64 + 1) * hands[i].bid;
    }

    println!("part1: {winnings}");

    // Part 2
    // switch the commented part
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, 4);
    }
}

#[derive(Debug)]
struct Hand {
    cards: [usize; 5],
    bid: i64,
}

impl Hand {
    // //part 1 ----------------------------------------------------------------
    // fn parse_string(s: &str) -> Self {
    //     let (card_part, bid_part) = s.split_once(" ").unwrap();
    //     let bid = bid_part.parse::<i64>().unwrap();
    //     let cards: Vec<usize> = card_part.chars()
    //                          .map(card_to_ord)
    //                          .collect();
    //     let cards = <[usize; 5]>::try_from(&cards[0..5]).unwrap();
    //
    //     Self {cards, bid}
    // }
    //
    // fn get_type(&self) -> usize {
    //     let reps = self.get_card_reps();
    //     hand_type(reps)
    // }


    //part 2 -------------------------------------------------------------------
    fn parse_string(s: &str) -> Self {
        let (card_part, bid_part) = s.split_once(" ").unwrap();
        let bid = bid_part.parse::<i64>().unwrap();
        let cards: Vec<usize> = card_part.chars()
                             .map(card_to_ord_joker)
                             .collect();
        let cards = <[usize; 5]>::try_from(&cards[0..5]).unwrap();

        Self {cards, bid}
    }

    fn get_type(&self) -> usize {
        let reps = self.get_card_reps_joker();
        hand_type(reps)
    }

    // common ------------------------------------------------------------------

    fn get_labels_count(&self) -> [usize; 13] {
        let mut labels_count = [0; 13];
        for c in self.cards {
            labels_count[c] += 1;
        }
        labels_count
    }

    fn get_card_reps(&self) -> [usize; 2] {
        let mut counts = self.get_labels_count();
        counts.sort();
        counts.reverse();
        <[usize; 2]>::try_from(&counts[0..2]).unwrap()
    }


    fn get_card_reps_joker(&self) -> [usize; 2] {
        let mut counts = self.get_labels_count();
        let joker_count = counts[12];
        counts[12] = 0;
        counts.sort();
        counts.reverse();
        let mut part_counts = <[usize; 2]>::try_from(&counts[0..2]).unwrap();
        part_counts[0] += joker_count;
        part_counts
    }

}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut order = self.get_type().cmp(&(other.get_type()));
        let mut n = 0;
        while order == Ordering::Equal {
            order = self.cards[n].cmp(&other.cards[n]);
            n += 1;
            if n > self.cards.len() {
                break;
            }
        }
        order
    }
}

fn card_to_ord(c: char) -> usize {
    "AKQJT98765432".find(c).unwrap()
}

fn card_to_ord_joker(c: char) -> usize {
    "AKQT98765432J".find(c).unwrap()
}

fn hand_type(reps: [usize; 2]) -> usize {
    let h_type = 10 * reps[0] + reps[1];
    match h_type {
        50 => 0, // five of a kind
        41 => 1, // four of a kind
        32 => 2, // full house
        31 => 3, // three of a kind
        22 => 4, // two pairs
        21 => 5, // one pair
        _  => 6, // high card
    }
}
