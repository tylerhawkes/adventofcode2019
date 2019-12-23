use std::collections::VecDeque;
use std::str::FromStr;

fn main() {
  let input =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day23.txt")).replace("\n", "");
  let mut network = input.parse::<Network>().unwrap();
  network.run();
  let Packet::Data(_, y) = network.nat_first_packet.unwrap();
  println!("First y value sent to 255 {:?}", y);
  let Packet::Data(_, y) = network.nat_last_packet.unwrap();
  println!("First y value repeated to 255: {:?}", y);
}

#[derive(Clone, Debug)]
struct Network {
  nics: Vec<Program<NIC>>,
  nat_last_packet: Option<Packet>,
  nat_first_packet: Option<Packet>,
}

impl Network {
  fn run(&mut self) {
    let mut _loop = 0;
    let mut idle_loops = 0;
    let mut last_nat_packet_sent_to_0 = None;
    'outer: loop {
      //println!("running loop {}", _loop);
      let mut sent_packet = false;
      for nic in 0..self.nics.len() {
        if self.run_nic(nic) {
          sent_packet = true;
        }
      }
      if sent_packet {
        idle_loops = 0;
      } else {
        idle_loops += 1;
      }
      if idle_loops > 10 {
        if let (Some(Packet::Data(_, y1)), Some(Packet::Data(_, y2))) =
          (self.nat_last_packet, last_nat_packet_sent_to_0)
        {
          if y1 == y2 {
            break;
          }
        }
        last_nat_packet_sent_to_0 = self.nat_last_packet;
        self.nics[0]
          .system
          .input
          .push_back(self.nat_last_packet.take().unwrap());
      }
      _loop += 1;
    }
  }
  fn run_nic(&mut self, nic: usize) -> bool {
    self.nics[nic].run_to_event();
    let mut sent_packet = false;
    while let Some((address, packet)) = self.nics[nic].system.output.pop_front() {
      //println!("Sending {:?} to {} from {}", packet, address, nic);
      if address == 255 {
        //println!("Sending packet {:?} to NAT", packet);
        self.nat_last_packet = Some(packet);
        self.nat_first_packet = self.nat_first_packet.or(Some(packet));
        sent_packet = true;
      } else {
        self.nics[address as usize].system.input.push_back(packet);
        sent_packet = true;
      }
    }
    sent_packet
  }
}

impl FromStr for Network {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let nics = (0..50)
      .map(|i| {
        let mut program = s.parse::<Program<NIC>>().unwrap();
        program.ensure_space(10000);
        program.system.stable_address = i;
        program.system.address = Some(i);
        program
      })
      .collect::<Vec<_>>();
    Ok(Self {
      nics,
      nat_last_packet: None,
      nat_first_packet: None,
    })
  }
}

#[derive(Copy, Clone, Debug)]
enum Packet {
  Data(I, I),
}

#[derive(Debug, Clone)]
struct NIC {
  address: Option<I>,
  sent_y: Option<I>,
  send_to_address: Option<I>,
  send_x: Option<I>,
  stable_address: I,
  input: VecDeque<Packet>,
  output: VecDeque<(I, Packet)>,
  already_sent_no_input: usize,
}

impl Default for NIC {
  fn default() -> Self {
    Self {
      address: None,
      sent_y: None,
      input: VecDeque::with_capacity(8),
      output: VecDeque::with_capacity(8),
      send_to_address: None,
      send_x: None,
      stable_address: 0,
      already_sent_no_input: 0,
    }
  }
}

impl System for NIC {
  fn send_input(&mut self) -> Event {
    if let Some(address) = self.address {
      self.address = None;
      //println!("Setting address to {}", address);
      return Event::Input(address);
    }
    if let Some(y) = self.sent_y {
      self.sent_y = None;
      return Event::Input(y);
    } else {
      match self.input.pop_front() {
        Some(Packet::Data(x, y)) => {
          self.already_sent_no_input = 0;
          //println!("Received packet {}, {} ({})", x, y, self.stable_address);
          self.sent_y = Some(y);
          return Event::Input(x);
        }
        None => {
          if self.already_sent_no_input < 10 {
            self.already_sent_no_input += 1;
            return Event::Input(-1);
          } else {
            self.already_sent_no_input = 0;
            return Event::AskForInputAgain;
          }
        }
      }
    }
  }

  fn take_output(&mut self, output: I) -> Event {
    if self.send_to_address.is_none() {
      self.send_to_address = Some(output);
      return Event::Output(output);
    } else if self.send_x.is_none() {
      self.send_x = Some(output);
      return Event::Output(output);
    } else {
      let send_address = self.send_to_address.unwrap();
      let x = self.send_x.unwrap();
      let y = output;
      //println!(
      //  "Sending ({}, {}) to {} from {}",
      //  x, y, send_address, self.stable_address
      //);
      self.output.push_back((send_address, Packet::Data(x, y)));
      self.send_to_address = None;
      self.send_x = None;
      return Event::BreakOnOutput(output);
    }
  }
}

type I = i64;

#[allow(unused)]
enum Event {
  Input(I),
  BreakOnInput(I),
  AskForInputAgain,
  Output(I),
  BreakOnOutput(I),
  Halted,
}

trait System: Default {
  fn send_input(&mut self) -> Event;
  fn take_output(&mut self, output: I) -> Event;
  fn end(&mut self) {}
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
  #[allow(unused)]
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
  }
  fn run_to_event(&mut self) -> Event {
    loop {
      let opcode = (&self.codes[self.position..(self.position + 4).min(self.codes.len())]).into();
      let event = self.apply(opcode);
      match event {
        Some(e @ Event::BreakOnInput(_)) | Some(e @ Event::BreakOnOutput(_)) => {
          self.move_position(opcode);
          return e;
        }
        Some(e @ Event::AskForInputAgain) => return e,
        Some(e @ Event::Halted) => {
          self.system.end();
          return e;
        }
        _ => self.move_position(opcode),
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
  fn apply(&mut self, opcode: OpCode) -> Option<Event> {
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
        let input = self.system.send_input();
        match input {
          Event::Input(i) | Event::BreakOnInput(i) => self.codes[p1_position] = i,
          Event::AskForInputAgain => {}
          _ => {}
        }
        return Some(input);
      }
      OpCode::Output(p1) => {
        let value = p1.value(&self.codes, relative_position);
        return Some(self.system.take_output(value));
      }
      OpCode::Break => return Some(Event::Halted),
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
      _ => {
        println!("Cannot parse code: {}", code);
        unreachable!()
      }
    }
  }
}
