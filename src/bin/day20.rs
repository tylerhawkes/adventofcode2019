use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

fn main() {
  let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day20.txt"));
  let maze = input.parse::<Maze>().unwrap();
  println!("Shortest path is {} steps", maze.find_shortest_path());
  println!(
    "Shortest recursive path is {} steps",
    maze.find_shortest_recursive_path()
  );
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
  x: i16,
  y: i16,
}

impl Coord {
  fn neighbors(self) -> [Self; 4] {
    let x = self.x;
    let y = self.y;
    [
      Self { x, y: y - 1 },
      Self { x, y: y + 1 },
      Self { x: x + 1, y },
      Self { x: x - 1, y },
    ]
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Tile {
  Start,
  End,
  InnerPortal(char, char),
  OuterPortal(char, char),
  Wall,
  Passage,
  Empty,
}

impl Tile {
  fn from_char(
    c: char,
    neighbors: [Option<char>; 4],
    coord: Coord,
    min_letter_coord: Coord,
    max_letter_coord: Coord,
  ) -> Self {
    match c {
      '#' => Self::Wall,
      '.' => Self::Passage,
      'A'..='Z' => {
        let inner = min_letter_coord.x + 2 < coord.x
          && coord.x < max_letter_coord.x - 2
          && min_letter_coord.y + 2 < coord.y
          && coord.y < max_letter_coord.y - 2;
        let (index, other) = neighbors
          .iter()
          .enumerate()
          .filter_map(|(i, c)| (*c).map(|a| (i, a)))
          .find(|(_, c)| 'A' <= *c && *c <= 'Z')
          .unwrap();
        match (inner, index) {
          (true, 0) => Tile::InnerPortal(other, c),
          (true, 1) => Tile::InnerPortal(c, other),
          (true, 2) => Tile::InnerPortal(c, other),
          (true, 3) => Tile::InnerPortal(other, c),
          (false, 0) => Tile::OuterPortal(other, c),
          (false, 1) => Tile::OuterPortal(c, other),
          (false, 2) => Tile::OuterPortal(c, other),
          (false, 3) => Tile::OuterPortal(other, c),
          _ => unreachable!(),
        }
      }
      ' ' => Self::Empty,
      _ => unreachable!(),
    }
  }
  fn is_portal(self) -> bool {
    match self {
      Self::InnerPortal(_, _) | Self::OuterPortal(_, _) => true,
      _ => false,
    }
  }
  fn is_passage(self) -> bool {
    Self::Passage == self
  }
}

#[derive(Clone, Debug)]
struct Maze {
  map: HashMap<Coord, Tile>,
  portals: HashMap<(char, char), [(Tile, Coord); 2]>,
  start: Coord,
  end: Coord,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Searcher {
  coord: Coord,
  dist: usize,
  tile: Tile,
  level: u8,
}

impl PartialOrd for Searcher {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(&other))
  }
}

impl Ord for Searcher {
  fn cmp(&self, other: &Self) -> Ordering {
    let level = self.level.cmp(&other.level).reverse();
    match level {
      Ordering::Equal => self.dist.cmp(&other.dist).reverse(),
      _ => level,
    }
  }
}

impl Maze {
  fn find_shortest_path(&self) -> usize {
    let mut to_search = BinaryHeap::with_capacity(self.map.len());
    let mut already_searched = HashSet::with_capacity(self.map.len());
    to_search.push(Searcher {
      coord: self.start,
      dist: 0,
      tile: Tile::Start,
      level: 0,
    });
    already_searched.insert(self.start);
    while !to_search.is_empty() {
      let next = to_search.pop().unwrap();
      //println!("Searching {:?}", next);
      if next.tile == Tile::End {
        return next.dist - 2;
      }
      for neighbor in next.coord.neighbors().iter() {
        if already_searched.contains(neighbor) {
          //println!("Already searched {:?}", neighbor);
          continue;
        }
        match self.map.get(neighbor) {
          Some(t @ Tile::Passage) | Some(t @ Tile::End) => {
            already_searched.insert(*neighbor);
            let s = Searcher {
              coord: *neighbor,
              dist: next.dist + 1,
              tile: *t,
              level: 0,
            };
            //println!("Adding {:?}", s);
            to_search.push(s);
          }
          Some(t @ Tile::InnerPortal(_, _)) | Some(t @ Tile::OuterPortal(_, _)) => {
            //println!(
            //  "***************************************************\nJumping to other side of {:?}\n***************************************************",
            //  t
            //);
            let (a, b) = match t {
              Tile::InnerPortal(a, b) => (a, b),
              Tile::OuterPortal(a, b) => (a, b),
              _ => unreachable!(),
            };
            let coords = self.portals.get(&(*a, *b)).unwrap();
            let coord = if coords[0].1 == *neighbor {
              coords[1]
            } else {
              coords[0]
            };
            already_searched.insert(coord.1);
            to_search.push(Searcher {
              coord: coord.1,
              dist: next.dist,
              tile: *t,
              level: 0,
            });
          }
          Some(Tile::Start) => unreachable!(),
          Some(Tile::Wall) | Some(Tile::Empty) | None => {}
        }
      }
    }
    0
  }
  fn find_shortest_recursive_path(&self) -> usize {
    let mut to_search = BinaryHeap::with_capacity(self.map.len());
    let mut already_searched = HashMap::<u8, HashSet<Coord>>::new();
    to_search.push(Searcher {
      coord: self.start,
      dist: 0,
      tile: Tile::Start,
      level: 0,
    });
    let mut level_0 = HashSet::new();
    level_0.insert(self.start);
    already_searched.insert(0, level_0);
    while !to_search.is_empty() {
      let next = to_search.pop().unwrap();
      //println!("Searching {:?}", next);
      if next.tile == Tile::End && next.level == 0 {
        return next.dist - 2;
      }
      for neighbor in next.coord.neighbors().iter() {
        if already_searched
          .get(&next.level)
          .unwrap()
          .contains(neighbor)
        {
          //println!("Already searched {:?}", neighbor);
          continue;
        }
        match self.map.get(neighbor) {
          Some(Tile::Start) | Some(Tile::End) if next.level > 0 => {}
          Some(Tile::OuterPortal(_, _)) if next.level == 0 => {}
          Some(t @ Tile::Passage) | Some(t @ Tile::End) => {
            match already_searched.entry(next.level) {
              Entry::Occupied(mut o) => {
                o.get_mut().insert(*neighbor);
              }
              Entry::Vacant(v) => {
                let mut searched = HashSet::new();
                searched.insert(*neighbor);
                v.insert(searched);
              }
            }
            let s = Searcher {
              coord: *neighbor,
              dist: next.dist + 1,
              tile: *t,
              level: next.level,
            };
            //println!("Adding {:?}", s);
            to_search.push(s);
          }
          Some(t @ Tile::InnerPortal(_, _)) | Some(t @ Tile::OuterPortal(_, _)) => {
            //println!(
            //    "***************************************************\nJumping to other side of {:?}\n***************************************************",
            //    t
            //  );
            let (a, b, level_increase) = match t {
              Tile::InnerPortal(a, b) => (a, b, true),
              Tile::OuterPortal(a, b) => (a, b, false),
              _ => unreachable!(),
            };
            let coords = self.portals.get(&(*a, *b)).unwrap();
            let coord = if coords[0].1 == *neighbor {
              coords[1]
            } else {
              coords[0]
            };
            let new_level = if level_increase {
              next.level + 1
            } else {
              next.level - 1
            };
            match already_searched.entry(new_level) {
              Entry::Occupied(mut o) => {
                o.get_mut().insert(coord.1);
              }
              Entry::Vacant(v) => {
                let mut searched = HashSet::new();
                searched.insert(coord.1);
                v.insert(searched);
              }
            }

            to_search.push(Searcher {
              coord: coord.1,
              dist: next.dist,
              tile: *t,
              level: new_level,
            });
          }
          Some(Tile::Start) => unreachable!(),
          Some(Tile::Wall) | Some(Tile::Empty) | None => {}
        }
      }
    }
    0
  }
}

impl FromStr for Maze {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let raw_map = s
      .lines()
      .enumerate()
      .flat_map(|(y, s)| {
        s.chars().enumerate().map(move |(x, c)| {
          (
            Coord {
              x: x as i16,
              y: y as i16,
            },
            c,
          )
        })
      })
      .collect::<HashMap<_, _>>();

    let xmin = raw_map
      .iter()
      .filter(|(_, c)| 'A' <= **c && **c <= 'Z')
      .map(|c| c.0.x)
      .min()
      .unwrap();
    let ymin = raw_map
      .iter()
      .filter(|(_, c)| 'A' <= **c && **c <= 'Z')
      .map(|c| c.0.y)
      .min()
      .unwrap();
    let xmax = raw_map
      .iter()
      .filter(|(_, c)| 'A' <= **c && **c <= 'Z')
      .map(|c| c.0.x)
      .max()
      .unwrap();
    let ymax = raw_map
      .iter()
      .filter(|(_, c)| 'A' <= **c && **c <= 'Z')
      .map(|c| c.0.y)
      .max()
      .unwrap();

    let min_letter_coord = Coord { x: xmin, y: ymin };
    let max_letter_coord = Coord { x: xmax, y: ymax };

    let map = raw_map
      .iter()
      .map(|(coord, c)| {
        let neighbors = coord.neighbors();
        let neighbors = [
          raw_map.get(&neighbors[0]).copied(),
          raw_map.get(&neighbors[1]).copied(),
          raw_map.get(&neighbors[2]).copied(),
          raw_map.get(&neighbors[3]).copied(),
        ];
        let tile = Tile::from_char(*c, neighbors, *coord, min_letter_coord, max_letter_coord);
        (*coord, tile)
      })
      .collect::<HashMap<_, _>>();

    //Drop off next neighbors of a portal.
    let mut cleaned_map = map
      .iter()
      .filter(|(c, t)| {
        if (*t).is_portal() {
          (*c)
            .neighbors()
            .iter()
            .filter_map(|c| map.get(c))
            .any(|t| (*t).is_passage())
        } else {
          true
        }
      })
      .map(|(c, t)| (*c, *t))
      .collect::<HashMap<_, _>>();

    let mut start = Coord { x: 0, y: 0 };
    let mut end = start;
    let mut portals = HashMap::<(char, char), [(Tile, Coord); 2]>::new();
    cleaned_map.iter_mut().for_each(|(c, t)| match t {
      Tile::OuterPortal('A', 'A') => {
        *t = Tile::Start;
        start = *c;
        //println!("start = {:?}", start);
      }
      Tile::OuterPortal('Z', 'Z') => {
        *t = Tile::End;
        end = *c;
        //println!("end = {:?}", end);
      }
      Tile::InnerPortal(a, b) | Tile::OuterPortal(a, b) => match portals.entry((*a, *b)) {
        Entry::Occupied(mut o) => {
          assert_eq!(o.get()[0], o.get()[1]);
          o.get_mut()[1] = (*t, *c);
          //println!("Found pair {:?}, {:?}", t, o.get());
        }
        Entry::Vacant(v) => {
          v.insert([(*t, *c), (*t, *c)]);
        }
      },
      _ => {}
    });

    Ok(Maze {
      map: cleaned_map,
      portals,
      start,
      end,
    })
  }
}

#[test]
fn test1() {
  let maze = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       "
    .parse::<Maze>()
    .unwrap();
  assert_eq!(23, maze.find_shortest_path());
}

#[test]
fn test2() {
  let maze = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               "
    .parse::<Maze>()
    .unwrap();
  assert_eq!(58, maze.find_shortest_path());
}

#[test]
fn test3() {
  let maze = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     "
    .parse::<Maze>()
    .unwrap();
  assert_eq!(396, maze.find_shortest_recursive_path());
}
