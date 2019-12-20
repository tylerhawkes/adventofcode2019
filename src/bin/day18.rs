use arrayvec::ArrayVec;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

fn main() {
  let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day18.txt"));
  let tunnels = input.parse::<Tunnels>().unwrap();
}

#[derive(Debug, Clone)]
struct Tunnels {
  map: Vec<Vec<Tunnel>>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord {
  x: usize,
  y: usize,
}

impl Coord {
  fn possible_moves(self) -> [Self; 4] {
    let x = self.x;
    let y = self.y;
    if x == 0 || y == 0 {
      panic!(format!("trying to evaluate moves at {}, {}", x, y));
    }
    [
      Self { x, y: y - 1 },
      Self { x, y: y + 1 },
      Self { x: x + 1, y },
      Self { x: x - 1, y },
    ]
  }
}

impl From<(usize, usize)> for Coord {
  #[inline(always)]
  fn from((x, y): (usize, usize)) -> Self {
    Self { x, y }
  }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Dist {
  dist: usize,
  behind_doors: ArrayVec<[char; 8]>,
  behind_keys: ArrayVec<[char; 8]>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Searcher {
  coord: Coord,
  dist: Dist,
}

impl Searcher {
  fn possible_moves(&self) -> [Self; 4] {
    let dist = Dist {
      dist: self.dist.dist + 1,
      behind_doors: self.dist.behind_doors.clone(),
      behind_keys: self.dist.behind_keys.clone(),
    };
    let [n, s, e, w] = self.coord.possible_moves();
    [
      Self {
        coord: n,
        dist: dist.clone(),
      },
      Self {
        coord: s,
        dist: dist.clone(),
      },
      Self {
        coord: e,
        dist: dist.clone(),
      },
      Self { coord: w, dist },
    ]
  }
  fn with_door(mut self, d: char) -> Self {
    self.dist.behind_doors.push(d);
    self
  }
  fn with_key(mut self, k: char) -> Self {
    self.dist.behind_keys.push(k);
    self
  }
}

impl PartialOrd for Searcher {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Searcher {
  fn cmp(&self, other: &Self) -> Ordering {
    self.dist.dist.cmp(&other.dist.dist).reverse()
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OptionSearcher {
  collected_keys: HashSet<char>,
  total_dist: usize,
  coord: Coord,
}

impl PartialOrd for OptionSearcher {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for OptionSearcher {
  fn cmp(&self, other: &Self) -> Ordering {
    self.total_dist.cmp(&other.total_dist).reverse()
  }
}

impl Tunnels {
  fn find_shortest_distance(&self) -> usize {
    let doors = self.doors();
    let keys = self.keys();

    let mut searcher = BinaryHeap::with_capacity(1000);
    searcher.push(OptionSearcher {
      collected_keys: HashSet::with_capacity(keys),
      total_dist: 0,
      coord: self.entrance(),
    });

    let mut searches = 0;
    let mut picked_up = HashSet::with_capacity(keys);
    let dist = while !searcher.is_empty() {
      searches += 1;
      picked_up.clear();
      let next = searcher.pop().unwrap();
      println!("Searching: {:?} to_search: {}", next, searcher.len());
      if next.collected_keys.len() == keys {
        println!(
          "Minimum dist {} found in {} searches",
          next.total_dist, searches
        );
        return next.total_dist;
      }
      let mut possible_moves =
        self.find_distances_to_all_keys_from_coord(next.coord, keys, &next.collected_keys);
      possible_moves.retain(|(k, _)| !next.collected_keys.contains(k));
      //get all keys that aren't behind doors but that are behind multiple keys first.
      possible_moves.sort_unstable_by_key(|(_, s)| {
        (s.dist.behind_doors.len(), Reverse(s.dist.behind_keys.len()))
      });
      //println!("possible moves: {:#?}", possible_moves);
      possible_moves.into_iter().for_each(|(c, s)| {
        if s.dist.behind_doors.is_empty() && !picked_up.contains(&c) {
          let mut collected_keys = next.collected_keys.clone();
          s.dist.behind_keys.iter().copied().for_each(|k| {
            picked_up.insert(k);
            collected_keys.insert(k);
          });
          let os = OptionSearcher {
            collected_keys,
            total_dist: next.total_dist + s.dist.dist,
            coord: s.coord,
          };
          //println!("Adding {:?} for searching", os);
          searcher.push(os);
        }
      })
    };
    0

    //      let tunnel_size = self.map.iter().map(|v| v.len()).sum::<usize>();
    //      println!(
    //        "Current position {:?}, total_dist: {}, collected_keys: {:?}",
    //        current_position, total_dist, collected_keys
    //      );
    //      //Search out all keys available given current keys
    //      let mut possible_moves =
    //        self.find_distances_to_all_keys_from_coord(current_position, keys, &collected_keys);
    //      possible_moves.retain(|m| !collected_keys.contains(&m.0));
    //
    //      // sort moves in closest order taking account of the doors
    //      possible_moves
    //        .sort_unstable_by_key(|m| m.1.dist.dist * (m.1.dist.behind_doors.len() + 1) * tunnel_size);
    //      println!("Possible moves: {:#?}", possible_moves);
    //
    //      let mut dependencies = HashMap::new();
    //      possible_moves.iter().for_each(|(c, s)| {
    //        dependencies.insert(
    //          *c,
    //          s.dist
    //            .behind_doors
    //            .iter()
    //            .copied()
    //            .map(|c| c.to_ascii_lowercase())
    //            .collect::<ArrayVec<[char; 24]>>(),
    //        );
    //      });
    //
    //      let mut visited = HashSet::new();
    //      let total_dependencies = dependencies
    //        .iter()
    //        .map(|(k, v)| {
    //          visited.clear();
    //          let mut depends = v.clone();
    //
    //          let mut i = 0;
    //          while i < depends.len() {
    //            let next = depends[i];
    //            if !visited.contains(&next) {
    //              depends.extend(dependencies.get(&next).unwrap().iter().copied());
    //            }
    //            visited.insert(next);
    //            i += 1;
    //          }
    //          // deduplicate
    //          depends.as_mut_slice().sort_unstable();
    //          let mut last = '!';
    //          depends.retain(|c| {
    //            let retain = *c != last;
    //            last = *c;
    //            retain
    //          });
    //
    //          (*k, depends)
    //        })
    //        .collect::<HashMap<char, ArrayVec<[char; 24]>>>();
    //
    //      println!("total_dependencies: {:#?}", total_dependencies);
    //
    //      let mut keys_in_group = HashSet::<char>::new();
    //      let mut key_groups = Vec::new();
    //      possible_moves.iter().rev().for_each(|(c, s)| {
    //        if !keys_in_group.contains(&c) {
    //          s.dist.behind_keys.iter().for_each(|k| {
    //            keys_in_group.insert(*k);
    //          });
    //          key_groups.push((*c, s.clone()));
    //        }
    //      });
    //
    //      println!("Dependencies: {:#?}", dependencies);
    //      println!("key_groups: {:#?}", key_groups);
    //
    //      let closest_key = possible_moves
    //        .iter()
    //        .find(|m| m.1.dist.behind_doors.is_empty())
    //        .unwrap();
    //      collected_keys.insert(closest_key.0);
    //      current_position = closest_key.1.coord;
    //      total_dist += closest_key.1.dist.dist;
    //    }
    //    total_dist
  }
  fn find_distances_to_all_keys_from_coord(
    &self,
    c: Coord,
    keys: usize,
    collected_keys: &HashSet<char>,
  ) -> Vec<(char, Searcher)> {
    let mut been_there = HashSet::with_capacity(self.map.len() * self.map[0].len());
    let mut to_search = BinaryHeap::with_capacity(1000);
    let mut possible_moves = Vec::with_capacity(keys);
    let dist = Dist {
      dist: 0,
      behind_doors: ArrayVec::new(),
      behind_keys: ArrayVec::new(),
    };
    been_there.insert(c);
    to_search.push(Searcher { coord: c, dist });

    //Search out all keys
    while !to_search.is_empty() {
      let mut next = to_search.pop().unwrap();
      if let Some(k) = self.map[next.coord.y][next.coord.x].is_key() {
        if !collected_keys.contains(&k) {
          next = next.with_key(k);
        }
        possible_moves.push((k, next.clone()));
      }

      'moves: for s in next.possible_moves().iter().cloned() {
        if !been_there.contains(&s.coord) {
          let s = match self.map[s.coord.y][s.coord.x].is_passable(&collected_keys) {
            Ok(Some(d)) => s.with_door(d),
            Ok(None) => s,
            Err(()) => continue 'moves,
          };
          been_there.insert(s.coord);
          to_search.push(s);
        }
      }
    }
    possible_moves
  }
  fn entrance(&self) -> Coord {
    for y in self.map.iter().enumerate() {
      for x in y.1.iter().copied().enumerate() {
        if x.1 == Tunnel::Entrance {
          return (x.0, y.0).into();
        }
      }
    }
    unreachable!()
  }
  fn doors(&self) -> usize {
    self
      .map
      .iter()
      .map(|v| v.iter().cloned().filter(|t| t.is_door().is_some()).count())
      .sum()
  }
  fn keys(&self) -> usize {
    self
      .map
      .iter()
      .map(|v| v.iter().cloned().filter(|t| t.is_key().is_some()).count())
      .sum()
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tunnel {
  Wall,
  Passage,
  Entrance,
  Door(char),
  Key(char),
}

impl Tunnel {
  fn is_passable(self, collected_keys: &HashSet<char>) -> Result<Option<char>, ()> {
    match self {
      Self::Passage | Self::Entrance | Self::Key(_) => Ok(None),
      Self::Door(d) => {
        if collected_keys.contains(&d.to_ascii_lowercase()) {
          Ok(None)
        } else {
          Ok(Some(d))
        }
      }
      Self::Wall => Err(()),
    }
  }
  fn is_door(self) -> Option<char> {
    if let Self::Door(d) = self {
      Some(d)
    } else {
      None
    }
  }
  fn is_key(self) -> Option<char> {
    if let Self::Key(c) = self {
      Some(c)
    } else {
      None
    }
  }
}

impl FromStr for Tunnels {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let map = s
      .lines()
      .filter(|s| !s.is_empty())
      .map(|l| l.chars().map(Tunnel::from_char).collect::<Vec<_>>())
      .collect::<Vec<_>>();
    Ok(Self { map })
  }
}

impl Tunnel {
  fn from_char(c: char) -> Self {
    match c {
      '#' => Self::Wall,
      '.' => Self::Passage,
      '@' => Self::Entrance,
      a if a.is_ascii_uppercase() => Self::Door(a),
      a if a.is_ascii_lowercase() => Self::Key(a),
      _ => unreachable!(),
    }
  }
}

#[test]
fn test_1() {
  let tunnels = "#########
#b.A.@.a#
#########"
    .parse::<Tunnels>()
    .unwrap();
  assert_eq!(8, tunnels.find_shortest_distance());
}

#[test]
fn test_2() {
  let tunnels = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
    .parse::<Tunnels>()
    .unwrap();
  assert_eq!(86, tunnels.find_shortest_distance());
}

#[test]
fn test_3() {
  let tunnels = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"
    .parse::<Tunnels>()
    .unwrap();
  assert_eq!(132, tunnels.find_shortest_distance());
}

#[test]
fn test_4() {
  let tunnels = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"
    .parse::<Tunnels>()
    .unwrap();
  assert_eq!(136, tunnels.find_shortest_distance());
}

#[test]
fn test_5() {
  let tunnels = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"
    .parse::<Tunnels>()
    .unwrap();
  assert_eq!(81, tunnels.find_shortest_distance());
}
