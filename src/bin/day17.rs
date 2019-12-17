use std::str::FromStr;

fn main() {
  let input =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day17.txt")).replace("\n", "");
  let mut program = input.parse::<Program<ASCII>>().unwrap();
  program.run();
  program.system.end();
  let mut program = input.parse::<Program<Input>>().unwrap();
  program.ensure_space(1_000_000);
  program.codes[0] = 2;
  program.run();
}

struct Input {
  commands: String,
}

impl Default for Input {
  fn default() -> Self {
    Self {
      commands: "A,C,C,B,A,C,B,A,C,B\nL,6,R,12,L,4,L,6\nL,6,L,10,L,10,R,6\nR,6,L,6,R,12\nn\n"
      //commands: "A,C,C,A,B,A,B,A,B,C\nR,6,R,6,R,8,L,10,L,4\nL,4,L,12,R,6,L,10\nR,6,L,10,R,8\nn\n"
        .to_string(),
    }
  }
}

impl System for Input {
  fn send_input(&mut self) -> Option<i64> {
    if !self.commands.is_empty() {
      let i = self.commands.remove(0);
      println!("sending back {}", i);
      Some(i as I)
    } else {
      None
    }
  }

  fn take_output(&mut self, output: i64) {
    if output <= 127 && output > 0 {
      print!("{}", output as u8 as char);
    } else {
      println!();
      println!("{}", output);
    }
  }

  fn end(&mut self) -> Option<i64> {
    println!("Finished");
    None
  }
}

#[derive(Debug, Clone, Default)]
struct ASCII {
  map: Vec<Vec<Scaffold>>,
}

#[derive(Copy, Clone, Debug)]
enum Scaffold {
  Hash,
  Dot,
  NewLine,
  Up,
  Down,
  Left,
  Right,
  Tumbling,
  Nothing(char),
}

impl Scaffold {
  fn is_scaffolding(self) -> bool {
    match self {
      Self::Up | Self::Down | Self::Left | Self::Right | Self::Hash => true,
      _ => false,
    }
  }
  fn is_drone(self) -> bool {
    match self {
      Self::Up | Self::Down | Self::Left | Self::Right => true,
      _ => false,
    }
  }
}

impl From<I> for Scaffold {
  fn from(i: I) -> Self {
    match i {
      10 => Self::NewLine,
      35 => Self::Hash,
      46 => Self::Dot,
      94 => Self::Up,
      60 => Self::Left,
      62 => Self::Right,
      118 => Self::Down,
      88 => Self::Tumbling,
      _ => {
        println!("{}", i);
        Self::Nothing(i as u8 as char)
      }
    }
  }
}

impl ASCII {
  fn find_shortest_path(&mut self) {
    let mut commands = Vec::new();
    let mut dp = (0, 0, Scaffold::Hash);
    self.map.iter().enumerate().for_each(|(y, v)| {
      v.iter().enumerate().for_each(|(x, s)| {
        if s.is_drone() {
          dp = (y, x, *s);
        }
      })
    });
    println!("Drone position is at {:?}", dp);

    let mut previous_dp;
    let mut finished = false;
    let mut movements = 0;
    while !finished {
      previous_dp = dp;
      match dp.2 {
        Scaffold::Left => {
          if dp.1.checked_sub(1).is_some() && self.map[dp.0][dp.1 - 1].is_scaffolding() {
            movements += 1;
            dp = (dp.0, dp.1 - 1, dp.2);
          } else if dp.0.checked_sub(1).is_some() && self.map[dp.0 - 1][dp.1].is_scaffolding() {
            commands.push(movements.to_string());
            movements = 0;
            commands.push("R".to_string());
            dp = (dp.0, dp.1, Scaffold::Up);
          } else if dp.0 + 1 < self.map.len() && self.map[dp.0 + 1][dp.1].is_scaffolding() {
            commands.push(movements.to_string());
            movements = 0;
            commands.push("L".to_string());
            dp = (dp.0, dp.1, Scaffold::Down);
          } else {
            commands.push(movements.to_string());
            finished = true;
          }
        }
        Scaffold::Right => {
          if dp.1 + 1 < self.map[dp.0].len() && self.map[dp.0][dp.1 + 1].is_scaffolding() {
            movements += 1;
            dp = (dp.0, dp.1 + 1, dp.2);
          } else if dp.0.checked_sub(1).is_some() && self.map[dp.0 - 1][dp.1].is_scaffolding() {
            commands.push(movements.to_string());
            movements = 0;
            commands.push("L".to_string());
            dp = (dp.0, dp.1, Scaffold::Up);
          } else if dp.0 + 1 < self.map.len() && self.map[dp.0 + 1][dp.1].is_scaffolding() {
            commands.push(movements.to_string());
            movements = 0;
            commands.push("R".to_string());
            dp = (dp.0, dp.1, Scaffold::Down);
          } else {
            commands.push(movements.to_string());
            finished = true;
          }
        }
        Scaffold::Up => {
          if dp.0.checked_sub(1).is_some() && self.map[dp.0 - 1][dp.1].is_scaffolding() {
            movements += 1;
            dp = (dp.0 - 1, dp.1, dp.2);
          } else if dp.1 + 1 < self.map[dp.0].len() && self.map[dp.0][dp.1 + 1].is_scaffolding() {
            commands.push(movements.to_string());
            movements = 0;
            commands.push("R".to_string());
            dp = (dp.0, dp.1, Scaffold::Right);
          } else if dp.1.checked_sub(1).is_some() && self.map[dp.0][dp.1 - 1].is_scaffolding() {
            commands.push(movements.to_string());
            movements = 0;
            commands.push("L".to_string());
            dp = (dp.0, dp.1, Scaffold::Left);
          } else {
            commands.push(movements.to_string());
            finished = true;
          }
        }
        Scaffold::Down => {
          if dp.0 + 1 < self.map.len() && self.map[dp.0 + 1][dp.1].is_scaffolding() {
            movements += 1;
            dp = (dp.0 + 1, dp.1, dp.2);
          } else if dp.1 + 1 < self.map[dp.0].len() && self.map[dp.0][dp.1 + 1].is_scaffolding() {
            commands.push(movements.to_string());
            movements = 0;
            commands.push("L".to_string());
            dp = (dp.0, dp.1, Scaffold::Right);
          } else if dp.1.checked_sub(1).is_some() && self.map[dp.0][dp.1 - 1].is_scaffolding() {
            commands.push(movements.to_string());
            movements = 0;
            commands.push("R".to_string());
            dp = (dp.0, dp.1, Scaffold::Left);
          } else {
            commands.push(movements.to_string());
            finished = true;
          }
        }
        _ => unreachable!(),
      }
      self.map[previous_dp.0][previous_dp.1] = Scaffold::Hash;
      self.map[dp.0][dp.1] = dp.2;
      //self.print();
      //println!("dp: {:?}, commands: {:?}", dp, commands);
      //std::thread::sleep(Duration::from_millis(200));
    }
    commands.remove(0);
    println!("Drone position is at {:?}", dp);
    self.print();
    println!(
      "{}",
      commands
        .iter()
        .fold("".to_string(), |a, b| format!("{}{}", a, b))
    );
    println!("{:?}", commands);
    self.break_down_commands(commands);
  }
  //R6R6R8L10L4R6L10R8R6L10R8R6R6R8L10L4L4L12R6L10R6R6R8L10L4L4L12R6L10R6R6R8L10L4L4L12R6L10R6L10R8
  //ACCABABABC
  //R6R6R8L10L4
  //L4L12R6L10
  //R6L10R8
  //n

  fn break_down_commands(&mut self, commands: Vec<String>) {}

  fn get_alignment_sums(&self) {
    let mut alignment_sums = 0;
    for y in 1..self.map.len() - 1 {
      for x in 1..self.map[y].len() - 1 {
        if self.map[y][x].is_scaffolding()
          && self.map[y][x - 1].is_scaffolding()
          && self.map[y][x + 1].is_scaffolding()
          && self.map[y - 1][x].is_scaffolding()
          && self.map[y + 1][x].is_scaffolding()
        {
          alignment_sums += y * x;
        }
      }
    }

    println!("Sum of alignment parameters = {}", alignment_sums);
  }

  fn print(&self) {
    println!("------------------------------------------------");
    self.map.iter().for_each(|v| {
      v.iter().for_each(|s| {
        let to_print = match s {
          Scaffold::Hash => '#',
          Scaffold::Dot => '.',
          Scaffold::Up => '^',
          Scaffold::Left => '<',
          Scaffold::Right => '>',
          Scaffold::Down => 'v',
          Scaffold::Tumbling => 'X',
          Scaffold::Nothing(c) => *c,
          _ => unreachable!(),
        };
        print!("{}", to_print);
      });
      println!();
    });
  }
}

impl System for ASCII {
  fn send_input(&mut self) -> Option<I> {
    println!("asked for input...");
    None
  }
  fn take_output(&mut self, output: I) {
    if self.map.is_empty() {
      self.map.push(Vec::with_capacity(1000));
    }
    let s = Scaffold::from(output);
    match s {
      Scaffold::NewLine => self.map.push(Vec::with_capacity(1000)),
      _ => self.map.last_mut().unwrap().push(s),
    }
  }

  fn end(&mut self) -> Option<I> {
    self.map.retain(|v| !v.is_empty());
    self.print();
    self.get_alignment_sums();
    self.find_shortest_path();
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
