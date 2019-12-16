use std::str::FromStr;
use std::time::Instant;

fn main() {
  let input =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day16.txt")).replace("\n", "");

  let fft = input.parse::<FFT>().unwrap();
  println!("{}", fft.calculate_phases(100));

  let fft_10_000 = input.repeat(10_000).parse::<FFT>().unwrap();
  println!("{}", fft_10_000.calculate_phases_fast(100));
}

struct PhasePatternIterator {
  base_phase: [i32; 4],
  phase_number: usize,
  base_phase_offset: usize,
  phase_count: usize,
}

impl PhasePatternIterator {
  fn new(phase_number: usize) -> Self {
    Self {
      base_phase: [0, 1, 0, -1],
      phase_number,
      base_phase_offset: 0,
      phase_count: 1,
    }
  }
}

impl Iterator for PhasePatternIterator {
  type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.phase_count == self.phase_number {
      self.phase_count = 0;
      self.base_phase_offset = (self.base_phase_offset + 1) % 4;
    }
    self.phase_count += 1;
    Some(self.base_phase[self.base_phase_offset])
  }
}

#[derive(Debug, Clone)]
struct FFT {
  elements: Vec<i32>,
  message_offset: usize,
}

impl FromStr for FFT {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let elements = s.chars().map(|c| c as i32 - '0' as i32).collect::<Vec<_>>();
    let message_offset = elements[..7]
      .iter()
      .fold(0_usize, |a, b| a * 10 + *b as usize);
    Ok(Self {
      elements,
      message_offset,
    })
  }
}

impl FFT {
  fn calculate_phase(&mut self) {
    let mut new_elements = self
      .elements
      .iter()
      .enumerate()
      .map(|(i, _)| {
        self
          .elements
          .iter()
          .zip(PhasePatternIterator::new(i + 1))
          .map(|(e, p)| e * p)
          .sum::<i32>()
          .abs()
          % 10
      })
      .collect::<Vec<_>>();
    self.elements = new_elements;
  }

  fn calculate_phase_fast(&mut self) {
    let new_elements = self
      .elements
      .iter()
      .copied()
      .scan(0, |s, e| {
        *s += e;
        Some(s.abs() % 10)
      })
      .collect::<Vec<_>>();
    self.elements = new_elements;
  }
  fn calculate_phases(mut self, count: usize) -> usize {
    for _ in 0..count {
      self.calculate_phase();
    }
    self.elements[..8]
      .iter()
      .fold(0, |a, b| a * 10 + *b as usize)
  }
  fn calculate_phases_fast(mut self, count: usize) -> usize {
    assert!(self.message_offset > self.elements.len() / 2);
    self.elements = self.elements[self.message_offset..]
      .iter()
      .rev()
      .cloned()
      .collect();

    for _ in 0..count {
      self.calculate_phase_fast();
    }
    self
      .elements
      .iter()
      .rev()
      .take(8)
      .fold(0, |a, b| a * 10 + *b as usize)
  }
}

#[test]
fn test() {
  let mut fft = "12345678".parse::<FFT>().unwrap();
  assert_eq!(01029498, fft.calculate_phases(4));
  fft = "80871224585914546619083218645595".parse().unwrap();
  assert_eq!(24176176, fft.calculate_phases(100));
  fft = "19617804207202209144916044189917".parse().unwrap();
  assert_eq!(73745418, fft.calculate_phases(100));
  fft = "69317163492948606335995924319873".parse().unwrap();
  assert_eq!(52432133, fft.calculate_phases(100));
}

#[test]
fn test_fast() {
  let mut fft = "03036732577212944063491565474664"
    .repeat(10_000)
    .parse::<FFT>()
    .unwrap();
  assert_eq!(84462026, fft.calculate_phases_fast(100));
  fft = "02935109699940807407585447034323"
    .repeat(10_000)
    .parse()
    .unwrap();
  assert_eq!(78725270, fft.calculate_phases_fast(100));
  fft = "03081770884921959731165446850517"
    .repeat(10_000)
    .parse()
    .unwrap();
  assert_eq!(53553731, fft.calculate_phases_fast(100));
}
