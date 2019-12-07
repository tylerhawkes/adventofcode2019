use std::str::FromStr;
use std::time::Instant;

fn main() {
  let start = Instant::now();
  let codes =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day07.txt")).replace("\n", "");
  let base_program = codes.parse::<Program>().unwrap();
  let max_thrust = optimize_thrust(&base_program);
  println!("Max thrust: {}", max_thrust);
  println!("took: {:?}", start.elapsed());
}

#[cfg(feature = "part-one")]
fn optimize_thrust(base: &Program) -> i32 {
  let mut best = i32::min_value();
  for a in 0..5 {
    for b in 0..5 {
      for c in 0..5 {
        for d in 0..5 {
          for e in 0..5 {
            if valid(a, b, c, d, e) {
              let val = run_amplifiers(base, a, b, c, d, e);
              if val > best {
                best = val;
              }
            }
          }
        }
      }
    }
  }
  best
}

#[cfg(feature = "part-two")]
fn optimize_thrust(base: &Program) -> i32 {
  let mut best = i32::min_value();
  for a in 5..10 {
    for b in 5..10 {
      for c in 5..10 {
        for d in 5..10 {
          for e in 5..10 {
            if valid(a, b, c, d, e) {
              let val = run_amplifiers(base, a, b, c, d, e);
              if val > best {
                best = val;
              }
            }
          }
        }
      }
    }
  }
  best
}

fn valid(a: i32, b: i32, c: i32, d: i32, e: i32) -> bool {
  a != b && a != c && a != d && a != e && b != c && b != d && b != e && c != d && c != e && d != e
}

#[cfg(feature = "part-one")]
fn run_amplifiers(base: &Program, a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
  let mut prog_a = base.clone().i(a).i(0);
  let mut prog_b = base.clone().i(b).i(prog_a.run()[0]);
  let mut prog_c = base.clone().i(c).i(prog_b.run()[0]);
  let mut prog_d = base.clone().i(d).i(prog_c.run()[0]);
  let mut prog_e = base.clone().i(e).i(prog_d.run()[0]);
  prog_e.run()[0]
}

#[cfg(feature = "part-two")]
fn run_amplifiers(base: &Program, a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
  let mut programs = vec![
    base.clone().name("a").i(a),
    base.clone().name("b").i(b),
    base.clone().name("c").i(c),
    base.clone().name("d").i(d),
    base.clone().name("e").i(e),
  ];

  let val = (0..5).cycle().try_fold(0, |input, index| {
    //println!("{}: Pushing {} to input", &programs[index].name, input);
    programs[index].input.push(input);
    let val = programs[index].run();
    //println!("{}: run returned {:?}", &programs[index].name, val);
    val
  });
  let val = val.unwrap_err();
  programs[4].output.last().cloned().unwrap()
}

#[derive(Clone, Debug)]
struct Program {
  name: String,
  codes: Vec<i32>,
  input: Vec<i32>,
  output: Vec<i32>,
  position: usize,
}

impl Program {
  #[cfg(feature = "part-two")]
  fn name(mut self, s: impl Into<String>) -> Self {
    self.name = s.into();
    self
  }
  #[cfg(feature = "part-one")]
  fn run(&mut self) -> &[i32] {
    loop {
      let opcode = (&self.codes[self.position..(self.position + 4).min(self.codes.len())]).into();
      if let OpCode::Break = opcode {
        break;
      }
      self.apply(opcode);
      self.move_position(opcode);
    }
    self.output.as_slice()
  }
  #[cfg(feature = "part-two")]
  fn run(&mut self) -> Result<i32, i32> {
    loop {
      let opcode = (&self.codes[self.position..(self.position + 4).min(self.codes.len())]).into();
      if let OpCode::Break = opcode {
        return Err(0);
      }
      let r = self.apply(opcode);
      self.move_position(opcode);
      if let Some(i) = r {
        //self.position = 0;
        return Ok(i);
      }
    }
  }
  fn move_position(&mut self, opcode: OpCode) {
    let codes = self.codes.as_slice();
    match opcode {
      OpCode::Add(_, _, _) | OpCode::Multiply(_, _, _) => self.position += 4,
      OpCode::Input(_) | OpCode::Output(_) => self.position += 2,
      OpCode::Break => unreachable!(),
      OpCode::LessThan(_, _, _) | OpCode::Equals(_, _, _) => self.position += 4,
      OpCode::JumpIfTrue(p1, p2) => {
        if p1.value(codes) != 0 {
          self.position = p2.value(codes) as usize;
        } else {
          self.position += 3;
        }
      }
      OpCode::JumpIfFalse(p1, p2) => {
        if p1.value(codes) == 0 {
          self.position = p2.value(codes) as usize;
        } else {
          self.position += 3
        }
      }
    };
    //println!("{}: Moving position to {}", self.name, self.position);
  }
  fn apply(&mut self, opcode: OpCode) -> Option<i32> {
    //println!("{}: Applying {:?}", self.name, opcode);
    let codes = self.codes.as_mut_slice();
    match opcode {
      OpCode::Add(p1, p2, p3) => codes[p3.position()] = p1.value(codes) + p2.value(codes),
      OpCode::Multiply(p1, p2, p3) => codes[p3.position()] = p1.value(codes) * p2.value(codes),
      OpCode::Input(p1) => codes[p1.position()] = self.input.remove(0),
      OpCode::Output(p1) => {
        self.output.push(p1.value(codes));
        return Some(p1.value(codes));
      }
      OpCode::Break => unreachable!(),
      OpCode::JumpIfTrue(_, _) | OpCode::JumpIfFalse(_, _) => {}
      OpCode::LessThan(p1, p2, p3) => {
        codes[p3.position()] = if p1.value(codes) < p2.value(codes) {
          1
        } else {
          0
        }
      }
      OpCode::Equals(p1, p2, p3) => {
        codes[p3.position()] = if p1.value(codes) == p2.value(codes) {
          1
        } else {
          0
        }
      }
    }
    None
  }
  fn i(mut self, i: i32) -> Self {
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
        s.parse::<i32>()
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
    })
  }
}

#[derive(Copy, Clone, Debug)]
enum ParameterMode {
  Position(i32),
  Immediate(i32),
}

impl ParameterMode {
  fn value(self, codes: &[i32]) -> i32 {
    let value = match self {
      Self::Position(i) => codes[i as usize],
      Self::Immediate(i) => i,
    };
    value
  }
  fn position(self) -> usize {
    match self {
      Self::Position(i) | Self::Immediate(i) => i as usize,
    }
  }
}

impl From<(i32, i32)> for ParameterMode {
  fn from((mode, value): (i32, i32)) -> Self {
    match mode {
      0 => Self::Position(value),
      1 => Self::Immediate(value),
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
}

impl From<&[i32]> for OpCode {
  fn from(codes: &[i32]) -> Self {
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
      99 => Self::Break,
      _ => unreachable!(),
    }
  }
}
