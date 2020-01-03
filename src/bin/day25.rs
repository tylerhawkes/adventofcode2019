use std::io::Read;
use std::str::FromStr;

fn main() {
  let input =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day25.txt")).replace("\n", "");
  let mut io = input.parse::<Program<Ship>>().unwrap();
  io.run();
}

//gmail: monolith, astrolabe, planetoid, fuel cell
//github: wreath, mug, astrolabe, sand

//enum Movements {
//  North,
//  South,
//  East,
//  West,
//}
//
//struct Map {}
//
//struct Output {
//  output: String,
//  room: String,
//  description: String,
//  doors_here_lead: Vec<String>,
//  items_here: Vec<String>,
//  extra: String,
//  command: bool,
//}
//
//#[derive(Default)]
//struct Droid {
//  output: Output,
//  map: Map,
//}

#[derive(Default)]
struct Ship;

impl System for Ship {
  fn send_input(&mut self) -> Option<i64> {
    let mut buf = [0];
    std::io::stdin().read(&mut buf).map(|_| buf[0] as i64).ok()
  }

  fn take_output(&mut self, output: i64) {
    if output > 127 {
      println!("{}", output);
    } else {
      let c = output as u8 as char;
      print!("{}", c);
    }
  }
}

type I = i64;

trait System: Default {
  fn send_input(&mut self) -> Option<I>;
  fn take_output(&mut self, output: I);
  fn end(&mut self) -> Option<I> {
    None
  }
  fn reset(&mut self) {}
}

#[derive(Clone, Debug)]
struct Program<S: System> {
  name: String,
  codes: Vec<I>,
  original_codes: Vec<I>,
  position: usize,
  relative_position: I,
  reset_after_run: bool,
  system: S,
}

impl<S: System> Program<S> {
  #[allow(unused)]
  fn name(mut self, s: impl Into<String>) -> Self {
    self.name = s.into();
    self
  }
  fn reset(&mut self) {
    self.position = 0;
    self.relative_position = 0;
    self
      .original_codes
      .reserve(self.codes.len() - self.original_codes.len());
    for _ in 0..self.codes.len() - self.original_codes.len() {
      self.original_codes.push(0);
    }
    self
      .codes
      .iter_mut()
      .zip(self.original_codes.iter().copied())
      .for_each(|(c, o)| {
        *c = o;
      });
    self.system.reset();
  }
  fn run(&mut self) -> Option<I> {
    loop {
      let opcode = (&self.codes[self.position..(self.position + 4).min(self.codes.len())]).into();
      if let OpCode::Break = opcode {
        break;
      }
      let cont = self.apply(opcode);
      self.move_position(opcode);
      if !cont {
        break;
      }
    }
    let output = self.system.end();
    if self.reset_after_run {
      self.reset();
    }
    output
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
      original_codes: codes.clone(),
      codes,
      position: 0,
      relative_position: 0,
      reset_after_run: false,
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
