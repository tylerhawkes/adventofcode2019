fn main() {
  //part 2 1377107 is too low
  let codes =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day05.txt")).replace("\n", "");
  let mut codes = codes
    .split(",")
    .filter_map(|s| {
      s.parse::<i32>()
        .map_err(|_| println!("Unable to parse {}", s))
        .ok()
    })
    .collect::<Vec<_>>();

  process_codes(&mut codes);
}

fn process_codes(codes: &mut [i32]) {
  let mut position = 0;
  loop {
    let opcode = (&codes[position..(position + 4).min(codes.len())]).into();
    if let OpCode::Break = opcode {
      return;
    }
    opcode.apply(codes);
    position = opcode.move_position(position, &codes);
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
  #[cfg(feature = "part-two")]
  JumpIfTrue(ParameterMode, ParameterMode),
  #[cfg(feature = "part-two")]
  JumpIfFalse(ParameterMode, ParameterMode),
  #[cfg(feature = "part-two")]
  LessThan(ParameterMode, ParameterMode, ParameterMode),
  #[cfg(feature = "part-two")]
  Equals(ParameterMode, ParameterMode, ParameterMode),
}

impl OpCode {
  fn move_position(self, current: usize, codes: &[i32]) -> usize {
    match self {
      Self::Add(_, _, _) | Self::Multiply(_, _, _) => current + 4,
      Self::Input(_) | Self::Output(_) => current + 2,
      Self::Break => unreachable!(),
      #[cfg(feature = "part-two")]
      Self::LessThan(_, _, _) | Self::Equals(_, _, _) => current + 4,
      #[cfg(feature = "part-two")]
      Self::JumpIfTrue(p1, p2) => {
        if p1.value(codes) != 0 {
          p2.value(codes) as usize
        } else {
          current + 3
        }
      }
      #[cfg(feature = "part-two")]
      Self::JumpIfFalse(p1, p2) => {
        if p1.value(codes) == 0 {
          p2.value(codes) as usize
        } else {
          current + 3
        }
      }
    }
  }
  fn apply(self, codes: &mut [i32]) {
    match self {
      Self::Add(p1, p2, p3) => codes[p3.position()] = p1.value(codes) + p2.value(codes),
      Self::Multiply(p1, p2, p3) => codes[p3.position()] = p1.value(codes) * p2.value(codes),
      #[cfg(feature = "part-one")]
      Self::Input(p1) => codes[p1.position()] = 1,
      #[cfg(feature = "part-two")]
      Self::Input(p1) => codes[p1.position()] = 5,
      Self::Output(p1) => println!("Output: {}", p1.value(codes)),
      Self::Break => unreachable!(),
      #[cfg(feature = "part-two")]
      Self::JumpIfTrue(_, _) | Self::JumpIfFalse(_, _) => {}
      #[cfg(feature = "part-two")]
      Self::LessThan(p1, p2, p3) => {
        codes[p3.position()] = if p1.value(codes) < p2.value(codes) {
          1
        } else {
          0
        }
      }
      #[cfg(feature = "part-two")]
      Self::Equals(p1, p2, p3) => {
        codes[p3.position()] = if p1.value(codes) == p2.value(codes) {
          1
        } else {
          0
        }
      }
    }
  }
}

impl From<&[i32]> for OpCode {
  fn from(codes: &[i32]) -> Self {
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
      #[cfg(feature = "part-two")]
      5 => Self::JumpIfTrue((param1, codes[1]).into(), (param2, codes[2]).into()),
      #[cfg(feature = "part-two")]
      6 => Self::JumpIfFalse((param1, codes[1]).into(), (param2, codes[2]).into()),
      #[cfg(feature = "part-two")]
      7 => Self::LessThan(
        (param1, codes[1]).into(),
        (param2, codes[2]).into(),
        (param3, codes[3]).into(),
      ),
      #[cfg(feature = "part-two")]
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

#[test]
fn test_code() {
  // all of these tests work as expected...
  let mut codes_1 = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
  //process_codes(&mut codes_1);
  let mut codes_2 = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
  //process_codes(&mut codes_2);
  let mut codes_3 = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
  //process_codes(&mut codes_3);
  let mut codes_4 = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
  //process_codes(&mut codes_4);
  let mut codes_5 = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
  //process_codes(&mut codes_5);
  let mut codes_6 = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
  //process_codes(&mut codes_6);
  let mut codes_7 = vec![
    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
    1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105,
    1, 46, 98, 99,
  ];
  process_codes(&mut codes_7);

  panic!("failing test");
}
