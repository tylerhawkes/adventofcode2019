use std::collections::LinkedList;
use std::str::FromStr;

const CARD_LEN: usize = 100003;

fn main() {
  let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day22.txt"));
  let mut card_shuffle = input.parse::<CardShuffle>().unwrap();
  card_shuffle.apply();
  let position = card_shuffle
    .deck
    .cards
    .iter()
    .enumerate()
    .find(|(_, c)| **c == 2019);
  println!("card 2019 is at position: {:?}", position);
  // See https://github.com/mcpower/adventofcode/blob/501b66084b0060e0375fc3d78460fb549bc7dfab/2019/22/a-improved.py
  // for an example of how to solve this as explained in
  // https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/ by `mcpower_`
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Technique {
  DealIntoNewStack,
  Cut(i16),
  DealWithIncrement(u16),
}

impl Technique {
  fn apply(self, from: &mut DeckOfCards, to: &mut DeckOfCards) {
    match self {
      Technique::DealIntoNewStack => {
        from
          .cards
          .iter()
          .rev()
          .zip(to.cards.iter_mut())
          .for_each(|(f, t)| {
            *t = *f;
          })
      }
      Technique::Cut(i) => {
        let split = if i >= 0 {
          i as usize
        } else {
          from.len - i.abs() as usize
        };
        let mut back = from.cards.split_off(split);
        let mut front = from.cards.split_off(0);
        back.append(&mut front);
        *from = DeckOfCards::new();
        *to = DeckOfCards {
          cards: back,
          len: to.len,
        }
      }
      Technique::DealWithIncrement(i) => {
        let mut cards = vec![0; from.len];
        let mut current_position = 0;
        for c in from.cards.iter().copied() {
          cards[current_position] = c;
          current_position += i as usize;
          if current_position > from.len {
            current_position %= from.len
          }
        }
        *to = DeckOfCards {
          cards: cards.into_iter().collect(),
          len: from.len,
        }
      }
    }
  }
}

#[derive(Clone, Debug)]
struct DeckOfCards {
  cards: LinkedList<u64>,
  len: usize,
}

impl DeckOfCards {
  fn new() -> Self {
    let mut cards = LinkedList::new();
    let len = CARD_LEN;
    for i in 0..len {
      cards.push_back(i as u64);
    }
    Self { cards, len }
  }
}

#[derive(Clone, Debug)]
struct CardShuffle {
  deck: DeckOfCards,
  new_deck: DeckOfCards,
  deals: Vec<Technique>,
}

impl CardShuffle {
  fn apply(&mut self) {
    for deal in self.deals.iter().copied() {
      deal.apply(&mut self.deck, &mut self.new_deck);
      std::mem::swap(&mut self.deck, &mut self.new_deck);
    }
  }
}

impl FromStr for CardShuffle {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let deals = s
      .lines()
      .map(|s| {
        if s.starts_with("cut") {
          Technique::Cut(s.split(" ").skip(1).next().unwrap().parse::<i16>().unwrap())
        } else if s.ends_with("stack") {
          Technique::DealIntoNewStack
        } else if s.starts_with("deal") {
          Technique::DealWithIncrement(s.split(" ").skip(3).next().unwrap().parse::<u16>().unwrap())
        } else {
          unreachable!()
        }
      })
      .collect::<Vec<_>>();
    Ok(Self {
      deck: DeckOfCards::new(),
      new_deck: DeckOfCards::new(),
      deals,
    })
  }
}
