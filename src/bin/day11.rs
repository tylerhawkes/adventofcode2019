use std::collections::HashMap;
use std::str::FromStr;

type I = i128;

fn main() {
  let codes =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day11.txt")).replace("\n", "");
  let mut program = codes.parse::<Program>().unwrap();
  program.ensure_space(1_000_000);
  let mut robot = Robot {
    program,
    panels_visited: HashMap::with_capacity(10_000),
    current_panel: Coord(0, 0),
    direction: Direction::North,
  };
  #[cfg(feature = "part-two")]
  {
    robot
      .panels_visited
      .insert(robot.current_panel, Color::White);
  }
  let panels_visited = robot.run();

  println!("Panels visited: {}", panels_visited);
  robot.paint();
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Color {
  Black,
  White,
}

impl From<I> for Color {
  fn from(i: i128) -> Self {
    match i {
      0 => Self::Black,
      1 => Self::White,
      _ => unreachable!(),
    }
  }
}

impl From<Color> for I {
  fn from(p: Color) -> Self {
    match p {
      Color::Black => 0,
      Color::White => 1,
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
  North,
  South,
  East,
  West,
}

impl Direction {
  fn turn(self, i: I) -> Self {
    //0 is left, 1 is right
    match (self, i) {
      (Self::North, 0) => Self::West,
      (Self::West, 0) => Self::South,
      (Self::South, 0) => Self::East,
      (Self::East, 0) => Self::North,
      (Self::North, 1) => Self::East,
      (Self::East, 1) => Self::South,
      (Self::South, 1) => Self::West,
      (Self::West, 1) => Self::North,
      (_, _) => unreachable!(),
    }
  }
  fn forward(self, c: Coord) -> Coord {
    match self {
      Self::North => Coord(c.0, c.1 + 1),
      Self::South => Coord(c.0, c.1 - 1),
      Self::East => Coord(c.0 + 1, c.1),
      Self::West => Coord(c.0 - 1, c.1),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord(isize, isize);

struct Robot {
  program: Program,
  panels_visited: HashMap<Coord, Color>,
  current_panel: Coord,
  direction: Direction,
}

impl Robot {
  fn run(&mut self) -> usize {
    loop {
      let color = self
        .panels_visited
        .get(&self.current_panel)
        .copied()
        .unwrap_or(Color::Black);
      self.program.input.push(color.into());
      match self.program.run_until_output() {
        Some(i) => {
          self.panels_visited.insert(self.current_panel, i.into());
        }
        None => break,
      }
      match self.program.run_until_output() {
        Some(i) => self.direction = self.direction.turn(i),
        None => break,
      }
      self.current_panel = self.direction.forward(self.current_panel);
    }
    self.panels_visited.len()
  }
  fn paint(&self) {
    let xmin = self.panels_visited.keys().map(|c| c.0).min().unwrap();
    let ymin = self.panels_visited.keys().map(|c| c.1).min().unwrap();
    let xmax = self.panels_visited.keys().map(|c| c.0).max().unwrap();
    let ymax = self.panels_visited.keys().map(|c| c.1).max().unwrap();
    println!("{}, {}, {}, {}", xmin, ymin, xmax, ymax);

    for y in (ymin..=ymax).rev() {
      for x in xmin..=xmax {
        let panel = self.panels_visited.get(&Coord(x, y)).copied();
        print!(
          "{}",
          match panel {
            Some(Color::Black) => " ",
            Some(Color::White) => "#",
            None => " ",
          }
        );
      }
      println!();
    }
  }
}

#[derive(Clone, Debug)]
struct Program {
  name: String,
  codes: Vec<I>,
  input: Vec<I>,
  output: Vec<I>,
  position: usize,
  relative_position: I,
}

impl Program {
  fn name(mut self, s: impl Into<String>) -> Self {
    self.name = s.into();
    self
  }
  fn run_until_output(&mut self) -> Option<I> {
    loop {
      let opcode = (&self.codes[self.position..(self.position + 4).min(self.codes.len())]).into();
      if let OpCode::Break = opcode {
        return None;
      }
      let output = self.apply(opcode);
      self.move_position(opcode);
      if let Some(i) = output {
        return Some(i);
      }
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
        self.codes[p1_position] = self.input.remove(0)
      }
      OpCode::Output(p1) => {
        self.output.push(p1.value(&self.codes, relative_position));
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
  fn i(mut self, i: I) -> Self {
    self.input.push(i);
    self
  }
}

impl FromStr for Program {
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
      input: vec![],
      output: vec![],
      position: 0,
      relative_position: 0,
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
