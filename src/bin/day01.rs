fn main() {
  let input: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day01.txt"));
  let iter = input.lines().map(|s| s.parse::<u32>().unwrap());

  let total_fuel: u32 = iter.clone().filter_map(fuel_required).sum();
  println!("Fuel required = {}", total_fuel);

  let total_added_fuel: u32 = iter.clone().map(total_fuel_required).sum();
  println!("Total fuel required = {}", total_added_fuel);
}

fn fuel_required(mass: u32) -> Option<u32> {
  (mass / 3).checked_sub(2)
}

fn total_fuel_required(mass: u32) -> u32 {
  let mut total = 0;
  let mut added_mass = mass;
  loop {
    if let Some(fuel_mass) = fuel_required(added_mass) {
      total += fuel_mass;
      added_mass = fuel_mass;
    } else {
      return total;
    }
  }
}

#[test]
fn test_fuel_required() {
  assert_eq!(fuel_required(12).unwrap(), 2);
  assert_eq!(fuel_required(14).unwrap(), 2);
  assert_eq!(fuel_required(1969).unwrap(), 654);
  assert_eq!(fuel_required(100756).unwrap(), 33583);
}

#[test]
fn test_total_fuel_required() {
  assert_eq!(total_fuel_required(12), 2);
  assert_eq!(total_fuel_required(14), 2);
  assert_eq!(total_fuel_required(1969), 966);
  assert_eq!(total_fuel_required(100756), 50346);
}
