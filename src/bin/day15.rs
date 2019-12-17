use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Error, Formatter};
use std::ops::Add;
use std::str::FromStr;

fn main() {
  let input =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day15.txt")).replace("\n", "");
  let mut program = input.parse::<Program<RepairDroid>>().unwrap();
  program.ensure_space(1_000_000);
  program.run();
}

#[derive(Copy, Clone, Debug)]
enum Movement {
  North,
  South,
  East,
  West,
}

impl Movement {
  fn iter() -> impl Iterator<Item = Self> {
    [
      Movement::North,
      Movement::East,
      Movement::South,
      Movement::West,
    ]
    .iter()
    .cloned()
  }
}

impl Into<I> for Movement {
  fn into(self) -> i64 {
    match self {
      Self::North => 1,
      Self::South => 2,
      Self::West => 3,
      Self::East => 4,
    }
  }
}

impl Add<(I, I)> for Movement {
  type Output = (I, I);

  fn add(self, rhs: (i64, i64)) -> Self::Output {
    match self {
      Self::North => (rhs.0, rhs.1 + 1),
      Self::South => (rhs.0, rhs.1 - 1),
      Self::East => (rhs.0 + 1, rhs.1),
      Self::West => (rhs.0 - 1, rhs.1),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum StatusCode {
  Wall,
  Moved,
  OxygenSystem,
}

impl From<I> for StatusCode {
  fn from(i: i64) -> Self {
    match i {
      0 => Self::Wall,
      1 => Self::Moved,
      2 => Self::OxygenSystem,
      _ => unreachable!(),
    }
  }
}

struct RepairDroid {
  map: HashMap<(I, I), StatusCode>,
  exhausted: HashSet<(I, I)>,
  position: (I, I),
  last_command: Movement,
  moves: I,
  searched: HashSet<(I, I)>,
}

impl Default for RepairDroid {
  fn default() -> Self {
    let mut map = HashMap::with_capacity(10_000);
    map.insert((0, 0), StatusCode::Moved);
    Self {
      map,
      exhausted: HashSet::with_capacity(10_000),
      position: (0, 0),
      last_command: Movement::North,
      moves: 0,
      searched: HashSet::new(),
    }
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Searcher {
  dist: usize,
  position: (I, I),
}

impl Ord for Searcher {
  fn cmp(&self, other: &Self) -> Ordering {
    self.dist.cmp(&other.dist).reverse()
  }
}

impl PartialOrd for Searcher {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Add<Movement> for Searcher {
  type Output = Self;

  fn add(self, rhs: Movement) -> Self::Output {
    Self {
      dist: self.dist + 1,
      position: rhs + self.position,
    }
  }
}

impl RepairDroid {
  fn determine_oxygen_fill_time(&mut self, oxygen_system_location: (I, I)) {
    let mut searcher = BinaryHeap::with_capacity(1000);
    searcher.push(Searcher {
      dist: 0,
      position: oxygen_system_location,
    });
    self.searched.clear();
    self.searched.insert(oxygen_system_location);
    let mut max_dist = 0;
    while !searcher.is_empty() {
      let next = searcher.pop().unwrap();
      if next.dist > max_dist {
        max_dist = next.dist;
      }
      for m in Movement::iter() {
        let new = next + m;
        let new_status = *self.map.get(&new.position).unwrap();
        if new_status == StatusCode::Moved && !self.searched.contains(&new.position) {
          searcher.push(new);
          self.searched.insert(new.position);
        }
      }
    }
    println!("Will take {} minutes to fill with oxygen", max_dist);
  }
  fn find_shortest_distance(&mut self) {
    let oxygen_system = *self
      .map
      .iter()
      .find(|(_, s)| **s == StatusCode::OxygenSystem)
      .unwrap()
      .0;
    let mut searcher = BinaryHeap::with_capacity(1000);
    let s = Searcher::default();
    searcher.push(s);
    self.searched.insert(s.position);
    while !searcher.is_empty() {
      let next = searcher.pop().unwrap();
      //println!("{:?}", next);
      for m in Movement::iter() {
        let new = next + m;
        if new.position == oxygen_system {
          println!("Found OxygenSystem after {} steps", new.dist);
          self.determine_oxygen_fill_time(oxygen_system);
          return;
        }
        let new_status = *self.map.get(&new.position).unwrap();
        if new_status == StatusCode::Moved && !self.searched.contains(&new.position) {
          searcher.push(new);
          self.searched.insert(new.position);
        }
      }
    }
    panic!("Unable to find path to oxygen system");
  }
  fn print(&self) {
    let mut xmin = I::max_value();
    let mut ymin = I::max_value();
    let mut xmax = I::min_value();
    let mut ymax = I::min_value();
    self.map.keys().for_each(|k| {
      if k.0 < xmin {
        xmin = k.0;
      } else if k.0 > xmax {
        xmax = k.0;
      }
      if k.1 < ymin {
        ymin = k.1;
      } else if k.1 > ymax {
        ymax = k.1;
      }
    });
    println!("----------------------------------------------------------");
    for y in (ymin..=ymax).rev() {
      for x in xmin..=xmax {
        let mut to_print = match self.map.get(&(x, y)) {
          Some(StatusCode::Moved) => {
            if self.searched.contains(&(x, y)) {
              '.'
            } else {
              ' '
            }
          }
          Some(StatusCode::Wall) => '#',
          Some(StatusCode::OxygenSystem) => 'O',
          None => ' ',
        };
        if (x, y) == self.position {
          to_print = 'D';
        } else if (x, y) == (0, 0) {
          to_print = 'X';
        }
        print!("{}", to_print);
      }
      println!()
    }
  }
}

impl System for RepairDroid {
  fn send_input(&mut self) -> Option<I> {
    // just dumbly try north then east then south then west
    let next_try = Movement::iter().find(|m| self.map.get(&(*m + self.position)).is_none());
    match next_try {
      Some(m) => {
        println!("Moving {:?}", m);
        self.last_command = m
      }
      None => {
        // Should only start filling in exhausted at dead ends
        self.exhausted.insert(self.position);
        match Movement::iter().find(|m| {
          let new_position = *m + self.position;
          self.map.get(&(new_position)).unwrap() == &StatusCode::Moved
            && !self.exhausted.contains(&new_position)
        }) {
          Some(move_back) => {
            println!("Moving back {:?}", move_back);
            self.last_command = move_back
          }
          None => return None,
        }
      }
    }
    self.moves += 1;
    //    if self.moves > 1000 {
    //      self.print();
    //      panic!();
    //    }
    Some(self.last_command.into())
  }

  fn take_output(&mut self, output: i64) {
    let status_code = StatusCode::from(output);
    match status_code {
      StatusCode::Wall => {
        self
          .map
          .insert(self.last_command + self.position, StatusCode::Wall);
        println!("Wall at {:?}", self.last_command + self.position);
      }
      StatusCode::Moved => {
        self.position = self.last_command + self.position;
        self.map.insert(self.position, StatusCode::Moved);
        //self.print();
        println!("Moved to {:?}", self.position);
      }
      StatusCode::OxygenSystem => {
        self.position = self.last_command + self.position;
        self.map.insert(self.position, StatusCode::OxygenSystem);
        //self.print();
        println!(
          "Found repair droid at {:?} after {} moves and {} backtracks",
          self.position,
          self.moves,
          self.searched.len(),
        );
      }
    }
  }

  fn end(&mut self) -> Option<I> {
    self.find_shortest_distance();
    None
  }
}

type I = i64;

trait System: Default {
  fn send_input(&mut self) -> Option<I>;
  fn take_output(&mut self, output: I);
  fn end(&mut self) -> Option<I>;
}

#[derive(Clone, Debug)]
struct Program<S: System> {
  name: String,
  codes: Vec<I>,
  position: usize,
  relative_position: I,
  system: S,
}

impl<S: System> Program<S> {
  fn name(mut self, s: impl Into<String>) -> Self {
    self.name = s.into();
    self
  }
  fn run(&mut self) -> Option<I> {
    loop {
      let opcode = (&self.codes[self.position..(self.position + 4).min(self.codes.len())]).into();
      if let OpCode::Break = opcode {
        return None;
      }
      let cont = self.apply(opcode);
      self.move_position(opcode);
      if !cont {
        break;
      }
    }
    self.system.end()
  }
  fn move_position(&mut self, opcode: OpCode) {
    let codes = self.codes.as_slice();
    match opcode {
      OpCode::Add(_, _, _) | OpCode::Multiply(_, _, _) => self.position += 4,
      OpCode::Input(_) | OpCode::Output(_) | OpCode::AdjustRelativeBase(_) => self.position += 2,
      OpCode::Break => unreachable!(),
      OpCode::LessThan(_, _, _) | OpCode::Equals(_, _, _) => self.position += 4,
      OpCode::JumpIfTrue(p1, p2) => {
        if p1.value(codes, self.relative_position) != 0 {
          self.position = p2.value(codes, self.relative_position) as usize;
        } else {
          self.position += 3;
        }
      }
      OpCode::JumpIfFalse(p1, p2) => {
        if p1.value(codes, self.relative_position) == 0 {
          self.position = p2.value(codes, self.relative_position) as usize;
        } else {
          self.position += 3
        }
      }
    };
    //println!("{}: Moving position to {}", self.name, self.position);
  }
  fn apply(&mut self, opcode: OpCode) -> bool {
    //println!(
    //  "{}: Applying {:?} (len: {})",
    //  self.name,
    //  opcode,
    //  self.codes.len(),
    //);
    let relative_position = self.relative_position;
    match opcode {
      OpCode::Add(p1, p2, p3) => {
        let p3_position = p3.position(relative_position);
        self.ensure_space(p3_position);
        self.codes[p3_position] =
          p1.value(&self.codes, relative_position) + p2.value(&self.codes, relative_position)
      }
      OpCode::Multiply(p1, p2, p3) => {
        let p3_position = p3.position(relative_position);
        self.ensure_space(p3_position);
        self.codes[p3_position] =
          p1.value(&self.codes, relative_position) * p2.value(&self.codes, relative_position)
      }
      OpCode::Input(p1) => {
        let p1_position = p1.position(relative_position);
        self.ensure_space(p1_position);
        match self.system.send_input() {
          Some(i) => self.codes[p1_position] = i,
          None => return false,
        }
      }
      OpCode::Output(p1) => {
        let value = p1.value(&self.codes, relative_position);
        self.system.take_output(value);
      }
      OpCode::Break => unreachable!(),
      OpCode::JumpIfTrue(_, _) | OpCode::JumpIfFalse(_, _) => {}
      OpCode::LessThan(p1, p2, p3) => {
        let p3_position = p3.position(relative_position);
        self.ensure_space(p3_position);
        self.codes[p3_position] =
          if p1.value(&self.codes, relative_position) < p2.value(&&self.codes, relative_position) {
            1
          } else {
            0
          }
      }
      OpCode::Equals(p1, p2, p3) => {
        let p3_position = p3.position(relative_position);
        self.ensure_space(p3_position);
        self.codes[p3_position] =
          if p1.value(&self.codes, relative_position) == p2.value(&self.codes, relative_position) {
            1
          } else {
            0
          }
      }
      OpCode::AdjustRelativeBase(p1) => {
        self.relative_position += p1.value(&self.codes, relative_position)
      }
    }
    true
  }
  fn ensure_space(&mut self, position: usize) {
    if self.codes.len() <= position {
      //println!("Extending from len {} to {}", self.codes.len(), position);
      self.codes.reserve(position - self.codes.len());
      while self.codes.len() <= position {
        self.codes.push(0);
      }
    }
  }
}

impl<S: System> FromStr for Program<S> {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let codes = s
      .split(",")
      .filter_map(|s| {
        s.parse::<I>()
          .map_err(|_| println!("Unable to parse {}", s))
          .ok()
      })
      .collect::<Vec<_>>();
    Ok(Self {
      name: "base".into(),
      codes,
      position: 0,
      relative_position: 0,
      system: S::default(),
    })
  }
}

#[derive(Copy, Clone, Debug)]
enum ParameterMode {
  Position(I),
  Immediate(I),
  Relative(I),
}

impl ParameterMode {
  fn value(self, codes: &[I], relative_position: I) -> I {
    let value = match self {
      Self::Position(i) => codes[i as usize],
      Self::Immediate(i) => i,
      Self::Relative(i) => codes[(i + relative_position) as usize],
    };
    //println!("Value of {:?} ({}) is {}", self, relative_position, value);
    value
  }
  fn position(self, relative_position: I) -> usize {
    let position = match self {
      Self::Position(i) | Self::Immediate(i) => i as usize,
      Self::Relative(i) => (i + relative_position) as usize,
    };
    //println!("Position of {:?} is {}", self, position);
    position
  }
}

impl From<(I, I)> for ParameterMode {
  fn from((mode, value): (I, I)) -> Self {
    match mode {
      0 => Self::Position(value),
      1 => Self::Immediate(value),
      2 => Self::Relative(value),
      _ => unreachable!(),
    }
  }
}

#[derive(Copy, Clone, Debug)]
enum OpCode {
  Add(ParameterMode, ParameterMode, ParameterMode),
  Multiply(ParameterMode, ParameterMode, ParameterMode),
  Input(ParameterMode),
  Output(ParameterMode),
  Break,
  JumpIfTrue(ParameterMode, ParameterMode),
  JumpIfFalse(ParameterMode, ParameterMode),
  LessThan(ParameterMode, ParameterMode, ParameterMode),
  Equals(ParameterMode, ParameterMode, ParameterMode),
  AdjustRelativeBase(ParameterMode),
}

impl From<&[I]> for OpCode {
  fn from(codes: &[I]) -> Self {
    //println!("Making OpCode from {:?}", codes);
    let i = codes[0];
    let code = i % 100;
    let param1 = i / 100 % 10;
    let param2 = i / 1000 % 10;
    let param3 = i / 10000 % 10;
    match code {
      1 => Self::Add(
        (param1, codes[1]).into(),
        (param2, codes[2]).into(),
        (param3, codes[3]).into(),
      ),
      2 => Self::Multiply(
        (param1, codes[1]).into(),
        (param2, codes[2]).into(),
        (param3, codes[3]).into(),
      ),
      3 => Self::Input((param1, codes[1]).into()),
      4 => Self::Output((param1, codes[1]).into()),
      5 => Self::JumpIfTrue((param1, codes[1]).into(), (param2, codes[2]).into()),
      6 => Self::JumpIfFalse((param1, codes[1]).into(), (param2, codes[2]).into()),
      7 => Self::LessThan(
        (param1, codes[1]).into(),
        (param2, codes[2]).into(),
        (param3, codes[3]).into(),
      ),
      8 => Self::Equals(
        (param1, codes[1]).into(),
        (param2, codes[2]).into(),
        (param3, codes[3]).into(),
      ),
      9 => Self::AdjustRelativeBase((param1, codes[1]).into()),
      99 => Self::Break,
      _ => unreachable!(),
    }
  }
}
