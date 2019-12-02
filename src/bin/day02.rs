fn main() {
  let ints = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day02.txt"));
  let ints = ints
    .split(",")
    .filter_map(|s| s.parse::<usize>().ok())
    .collect::<Vec<_>>();
  let mut i = ints.clone();
  i[1] = 12;
  i[2] = 2;
  process_ints(&mut i);
  println!("Value as position 0 is {}", i[0]);
  println!("Part two ints is {:?}", part_two(&ints));
}

fn process_ints(ints: &mut [usize]) {
  let mut position = 0;
  loop {
    let result = match ints[position] {
      99 => break,
      2 => ints[ints[position + 1]] * ints[ints[position + 2]],
      1 => ints[ints[position + 1]] + ints[ints[position + 2]],
      _ => panic!("Invalid code {} at position {}", ints[position], position),
    };
    let third_position = ints[position + 3];
    ints[third_position] = result;
    position += 4;
  }
}

fn part_two(ints: &[usize]) -> (usize, usize) {
  for noun in 0..100 {
    for verb in 0..100 {
      let mut i = ints.to_vec();
      i[1] = noun;
      i[2] = verb;
      process_ints(&mut i);
      if i[0] == 19690720 {
        return (noun, verb);
      }
    }
  }
  panic!("Did not find valid noun or verb");
}

#[test]
fn test_process_ints() {
  let mut ints = vec![1, 0, 0, 0, 99];
  process_ints(&mut ints);
  assert_eq!(ints, vec![2, 0, 0, 0, 99]);
  let mut ints = vec![2, 3, 0, 3, 99];
  process_ints(&mut ints);
  assert_eq!(ints, vec![2, 3, 0, 6, 99]);
  let mut ints = vec![2, 4, 4, 5, 99, 0];
  process_ints(&mut ints);
  assert_eq!(ints, vec![2, 4, 4, 5, 99, 9801]);
  let mut ints = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
  process_ints(&mut ints);
  assert_eq!(ints, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
}
