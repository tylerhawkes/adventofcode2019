use std::collections::HashSet;
use std::str::FromStr;

fn main() {
  let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day24.txt"));
  let eris = input.parse::<Eris>().unwrap();
  println!(
    "biodiversity rating for first layout to appear twice: {}",
    eris.find_first_repeat().bugs
  );
  println!(
    "Number of bugs after 200 recursions is: {}",
    eris.find_recursive_bugs(200)
  );
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Eris {
  bugs: u32,
}

impl std::fmt::Display for Eris {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    "----------\n".fmt(f)?;
    for line in 0..5 {
      for i in 0..5 {
        if self.bugs & 1 << (line * 5 + i) > 0 {
          '#'.fmt(f)?;
        } else {
          '.'.fmt(f)?;
        }
      }
      '\n'.fmt(f)?;
    }
    Ok(())
  }
}

impl Eris {
  fn new() -> Self {
    Self { bugs: 0 }
  }
  // 0 indexed so 0..=24
  fn neighbors(self, i: u32) -> u32 {
    let up_mask = match i / 5 {
      0 => 0,
      1..=4 => 1 << (i - 5),
      _ => unreachable!(),
    };
    let down_mask = match i / 5 {
      0..=3 => 1 << (i + 5),
      4 => 0,
      _ => unreachable!(),
    };
    let left_mask = match i % 5 {
      0 => 0,
      1..=4 => 1 << i - 1,
      _ => unreachable!(),
    };
    let right_mask = match i % 5 {
      0..=3 => 1 << i + 1,
      4 => 0,
      _ => unreachable!(),
    };
    up_mask | down_mask | left_mask | right_mask
  }

  fn alive_neighbors(self, i: u32) -> u32 {
    self.neighbors(i) & self.bugs
  }

  fn tick(self) -> Self {
    let mut bugs = self.bugs;
    for i in 0..25 {
      let bug = self.bugs & (1 << i) > 0;
      let neighbor_count = self.alive_neighbors(i).count_ones();
      if bug {
        if neighbor_count != 1 {
          bugs &= u32::max_value() ^ (1 << i);
        }
      } else {
        if neighbor_count == 1 || neighbor_count == 2 {
          bugs |= 1 << i;
        }
      }
    }
    Self { bugs }
  }

  fn find_first_repeat(self) -> Self {
    let mut set = HashSet::with_capacity(100_000);
    let mut tick = self;
    //println!("{}", tick);
    while !set.contains(&tick) {
      set.insert(tick);
      tick = tick.tick();
      //println!("{}", tick);
    }
    tick
  }

  fn recursive_neighbors(self, i: u32) -> (u32, u32, u32) {
    if i == 12 {
      return (0, 0, 0);
    }
    let same_level_neighbors = self.neighbors(i) & (u32::max_value() ^ (1 << 12));
    let inner_level_neighbors: u32 = match i {
      7 => (0_u32..5).map(|n| 1 << n).sum(),
      11 => (0_u32..5).map(|n| 1 << (n * 5)).sum(),
      13 => (0_u32..5).map(|n| 1 << (n * 5 + 4)).sum(),
      17 => (20_u32..25).map(|n| 1 << n).sum(),
      _ => 0,
    };
    let (up, down, left, right) = (1 << 7, 1 << 17, 1 << 11, 1 << 13);
    let outer_level_neighbors = match i {
      0 => up | left,
      1..=3 => up,
      4 => up | right,
      5 | 10 | 15 => left,
      9 | 14 | 19 => right,
      20 => down | left,
      21..=23 => down,
      24 => down | right,
      _ => 0,
    };
    (
      inner_level_neighbors,
      same_level_neighbors,
      outer_level_neighbors,
    )
  }

  fn tick_recursive(self, inner: Self, outer: Self) -> Self {
    let mut bugs = self.bugs;

    for i in 0..25 {
      if i == 12 {
        continue;
      }
      let (inner_neighbors, self_neighbors, outer_neighbors) = self.recursive_neighbors(i);
      let bug = self.bugs & (1 << i) > 0;
      let neighbor_count = (inner_neighbors & inner.bugs).count_ones()
        + (self_neighbors & self.bugs).count_ones()
        + (outer_neighbors & outer.bugs).count_ones();
      if bug {
        if neighbor_count != 1 {
          bugs &= u32::max_value() ^ (1 << i);
        }
      } else {
        if neighbor_count == 1 || neighbor_count == 2 {
          bugs |= 1 << i;
        }
      }
    }
    Self { bugs }
  }

  fn find_recursive_bugs(self, n: usize) -> usize {
    let mut levels = Vec::with_capacity(200);
    levels.push(self);
    //println!("{}", self);
    for _ in 0..n {
      if levels.first().unwrap().bugs > 0 {
        levels.insert(0, Eris::new());
      }
      if levels.last().unwrap().bugs > 0 {
        levels.push(Eris::new());
      }
      let mut new_levels = levels
        .windows(3)
        .map(|a| a[1].tick_recursive(a[0], a[2]))
        .collect::<Vec<_>>();
      new_levels.insert(0, levels[0].tick_recursive(Eris::new(), levels[1]));
      new_levels
        .push(levels[levels.len() - 1].tick_recursive(levels[levels.len() - 2], Eris::new()));
      levels = new_levels;
      //println!("----------------------------------------------------------");
      //levels.iter().enumerate().for_each(|(i, e)| {
      //  println!("{} {}", i, e);
      //});
    }

    levels.iter().map(|e| e.bugs.count_ones() as usize).sum()
  }
}

impl FromStr for Eris {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let bugs = s
      .lines()
      .enumerate()
      .map(|(i, l)| {
        l.chars()
          .enumerate()
          .map(|(j, c)| match c {
            '.' => 0_u32,
            '#' => 1 << i * 5 + j,
            _ => unreachable!(),
          })
          .sum::<u32>()
      })
      .sum();
    Ok(Self { bugs })
  }
}

#[test]
fn test_neighbors() {
  let eris = Eris { bugs: !0 };

  assert_eq!(
    vec![2, 3, 3, 3, 2, 3, 4, 4, 4, 3, 3, 4, 4, 4, 3, 3, 4, 4, 4, 3, 2, 3, 3, 3, 2],
    (0..25)
      .map(|i| eris.alive_neighbors(i).count_ones())
      .collect::<Vec<_>>()
  );
}

#[test]
fn test_recursive_neighbors() {
  let eris = Eris { bugs: !0 };
  assert_eq!(
    vec![4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 8, 0, 8, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4,],
    (0..25)
      .map(|i| {
        let (a, b, c) = eris.recursive_neighbors(i);
        a.count_ones() + b.count_ones() + c.count_ones()
      })
      .collect::<Vec<_>>()
  );

  assert_eq!(
    vec![
      (0, 2, 2),
      (0, 3, 1),
      (0, 3, 1),
      (0, 3, 1),
      (0, 2, 2),
      (0, 3, 1),
      (0, 4, 0),
      (5, 3, 0),
      (0, 4, 0),
      (0, 3, 1),
      (0, 3, 1),
      (5, 3, 0),
      (0, 0, 0),
      (5, 3, 0),
      (0, 3, 1),
      (0, 3, 1),
      (0, 4, 0),
      (5, 3, 0),
      (0, 4, 0),
      (0, 3, 1),
      (0, 2, 2),
      (0, 3, 1),
      (0, 3, 1),
      (0, 3, 1),
      (0, 2, 2),
    ],
    (0..25)
      .map(|i| {
        let (a, b, c) = eris.recursive_neighbors(i);
        (a.count_ones(), b.count_ones(), c.count_ones())
      })
      .collect::<Vec<_>>()
  );
}

#[test]
fn test_tick_recursive() {
  let inner = Eris::new();
  let outer = Eris::new();
  let mask = u32::max_value() ^ (1 << 12);
  for bugs in 0..1 << 25 - 1 {
    if bugs & (1 << 12) > 0 {
      continue;
    }
    let e = Eris { bugs };
    if e.tick().bugs & mask != e.tick_recursive(inner, outer).bugs {
      println!("{}", e);
      println!("{}", e.tick());
      println!("{}", e.tick_recursive(inner, outer));
      assert_eq!(e.tick(), e.tick_recursive(inner, outer));
    }
  }
}

#[test]
fn test_recursion() {
  let eris = "....#
#..#.
#..##
..#..
#...."
    .parse::<Eris>()
    .unwrap();
  assert_eq!(99, eris.find_recursive_bugs(10).0)
}
