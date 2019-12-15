use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

fn main() {
  let input =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day13.txt")).replace("\n", "");
  let mut program = input.parse::<Program<Game>>().unwrap();
  program.ensure_space(1_000_000);

  #[cfg(feature = "part-one")]
  {
    let block_tiles = program.run();
    println!("block tiles: {}", program.system.count_tiles(Tile::Block));
  }
  #[cfg(feature = "part-two")]
  {
    program.codes[0] = 2;
    program.run();
    println!("Score: {}", program.system.score);
  }
}

#[derive(Debug, Clone)]
struct Game {
  screen: HashMap<(I, I), Tile>,
  score: I,
  paddle: (I, I),
  ball: (I, I),
  program_output: Vec<I>,
}

impl Game {
  fn apply_output(&mut self, output: I) {
    self.program_output.push(output);
    if self.program_output.len() % 3 != 0 {
      return;
    }
    let mut output = self.program_output.drain(..);
    while let (Some(x), Some(y), Some(tile)) = (output.next(), output.next(), output.next()) {
      if x == -1 && y == 0 {
        println!("Updating score to: {}", tile);
        self.score = tile;
        continue;
      }
      let tile = Tile::from(tile);
      self.screen.insert((x, y), tile);
      match tile {
        Tile::HorizontalPaddle => self.paddle = (x, y),
        Tile::Ball => self.ball = (x, y),
        _ => {}
      }
    }
  }
  fn count_tiles(&self, tile: Tile) -> usize {
    self.screen.values().filter(|t| **t == tile).count()
  }
  fn print(&self) {
    if self.screen.is_empty() {
      return;
    }
    let xmin = self.screen.keys().map(|k| k.0).min().unwrap();
    let ymin = self.screen.keys().map(|k| k.1).min().unwrap();
    let xmax = self.screen.keys().map(|k| k.0).max().unwrap();
    let ymax = self.screen.keys().map(|k| k.1).max().unwrap();
    println!("bounds: {}, {}, {}, {}", xmin, ymin, xmax, ymax);
    for y in ymin..=ymax {
      for x in xmin..=xmax {
        print!("{}", self.screen.get(&(x, y)).unwrap_or(&Tile::Empty));
      }
      println!();
    }
  }
}

impl Default for Game {
  fn default() -> Self {
    Self {
      screen: HashMap::with_capacity(1000),
      score: 0,
      paddle: (0, 0),
      ball: (0, 0),
      program_output: Vec::with_capacity(10000),
    }
  }
}

impl System for Game {
  fn send_input(&mut self) -> i64 {
    let input = (self.ball.0 - self.paddle.0).signum();
    //println!("returning input: {}", input);
    input
  }

  fn take_output(&mut self, output: i64) {
    self.apply_output(output);
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
  Empty,
  Wall,
  Block,
  HorizontalPaddle,
  Ball,
}

impl From<I> for Tile {
  fn from(i: i64) -> Self {
    match i {
      0 => Self::Empty,
      1 => Self::Wall,
      2 => Self::Block,
      3 => Self::HorizontalPaddle,
      4 => Self::Ball,
      _ => unreachable!(),
    }
  }
}

impl Display for Tile {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    let c = match self {
      Self::Empty => " ",
      Self::Wall => "|",
      Self::Block => "#",
      Self::HorizontalPaddle => "_",
      Self::Ball => "O",
    };
    c.fmt(f)
  }
}

type I = i64;

trait System: Default {
  fn send_input(&mut self) -> I;
  fn take_output(&mut self, output: I);
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
      let output = self.apply(opcode);
      self.move_position(opcode);
      //      if let Some(i) = output {
      //        return Some(i);
      //      }
    }
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
  fn apply(&mut self, opcode: OpCode) -> Option<I> {
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
        self.codes[p1_position] = self.system.send_input();
      }
      OpCode::Output(p1) => {
        let value = p1.value(&self.codes, relative_position);
        self.system.take_output(value);
        return Some(p1.value(&self.codes, relative_position));
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
    None
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
