use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

fn main() {
  let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day14.txt"));
  let mut fuel_calc = FuelCalculator::from_str(input);
  let units_for_fuel = fuel_calc.calculate_ore_for_fuel(1);
  println!("{:?}", units_for_fuel);
  let fuel_from_1_trillion_ore = fuel_calc.calculate_fuel_for_1_trillion_ore();
  println!("{:?}", fuel_from_1_trillion_ore);
}

#[derive(Copy, Clone, Debug)]
struct ChemicalUnit<'a> {
  name: &'a str,
  units: u64,
}

impl<'a> ChemicalUnit<'a> {
  fn is_ore(&self) -> bool {
    self.name == "ORE"
  }
  fn with_units(&self, units: u64) -> Self {
    Self {
      name: self.name,
      units,
    }
  }
  fn from_str(s: &'a str) -> Self {
    let mut v = s.split(" ").filter(|p| p != &"");
    let units = v.next().unwrap().parse::<u64>().unwrap();
    let name = v.next().unwrap();
    Self { name, units }
  }
}

impl<'a> Hash for ChemicalUnit<'a> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.name.hash(state)
  }
}

impl<'a> PartialEq for ChemicalUnit<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.name.eq(other.name)
  }
}

impl<'a> Eq for ChemicalUnit<'a> {}

impl<'a> Ord for ChemicalUnit<'a> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.name.cmp(other.name)
  }
}

impl<'a> PartialOrd for ChemicalUnit<'a> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Debug, Clone)]
struct FuelCalculator<'a> {
  map: Arc<HashMap<&'a str, (ChemicalUnit<'a>, Vec<ChemicalUnit<'a>>)>>,
  available: HashMap<&'a str, u64>,
  ore_used: u64,
  fuel_created: u64,
}

impl<'a> FuelCalculator<'a> {
  fn calculate_fuel_for_1_trillion_ore(&mut self) -> u64 {
    const TRILLION: u64 = 1_000_000_000_000;
    let ore = self.calculate_ore_for_fuel(10);
    let mut min = TRILLION / (ore * 100);
    let mut max = TRILLION / (ore / 100);
    while min + 1 != max {
      //println!("min: {}, max: {}", min, max);
      let mid = (min + max) / 2;
      let ore = self.calculate_ore_for_fuel(mid);
      if ore < TRILLION {
        min = mid;
      } else {
        max = mid;
      }
    }
    min
  }
  fn calculate_ore_for_fuel(&mut self, number_of_fuel: u64) -> u64 {
    self.available.clear();
    let needed = self.calculate_recipe_for_chemical_unit(&ChemicalUnit {
      name: "FUEL",
      units: number_of_fuel,
    });
    needed
  }

  fn calculate_recipe_for_chemical_unit(&mut self, needed: &ChemicalUnit<'a>) -> u64 {
    if needed.is_ore() {
      //println!("returning {} ore units", needed.units);
      return needed.units;
    }
    let available = match self.available.entry(needed.name) {
      Entry::Occupied(o) => *o.get(),
      Entry::Vacant(_) => 0,
    };
    let map_clone = self.map.clone();
    let mut recipe_units = 0;
    let mut needed_ore = 0;
    let (recipe, recipe_chemicals) = map_clone.get(needed.name).unwrap();
    while recipe_units + available < needed.units {
      let scale = (((needed.units - recipe_units) / recipe.units).max(1) - 1).max(1);
      needed_ore += recipe_chemicals
        .into_iter()
        .map(|recipe_chemical| {
          self.calculate_recipe_for_chemical_unit(
            &recipe_chemical.with_units(recipe_chemical.units * scale),
          )
        })
        .sum::<u64>();
      recipe_units += recipe.units * scale;
    }
    if recipe_units + available >= needed.units {
      match self.available.entry(needed.name) {
        Entry::Occupied(mut o) => {
          *o.get_mut() = recipe_units + available - needed.units;
        }
        Entry::Vacant(v) => {
          v.insert(recipe_units + available - needed.units);
        }
      }
    }
    //println!("Created {} units for {:?}", recipe_units, needed);
    needed_ore
  }

  fn from_str(s: &'a str) -> Self {
    let map = s
      .lines()
      .map(|l| {
        let mut v = l.split("=>");
        let sources = v
          .next()
          .unwrap()
          .split(",")
          .map(|c| ChemicalUnit::from_str(c))
          .collect::<Vec<_>>();
        let chemical = ChemicalUnit::from_str(v.next().unwrap());
        (chemical.name, (chemical, sources))
      })
      .collect::<HashMap<_, _>>();
    Self {
      available: HashMap::with_capacity(map.len()),
      map: Arc::new(map),
      ore_used: 0,
      fuel_created: 0,
    }
  }
}

impl<'a> From<&'a str> for FuelCalculator<'a> {
  fn from(s: &'a str) -> Self {
    Self::from_str(s)
  }
}

#[test]
fn test_ore_to_fuel() {
  let mut fuel_calc: FuelCalculator = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"
    .into();

  //println!("{:#?}", fuel_calc);
  //assert_eq!(31, fuel_calc.calculate_ore_for_fuel());

  fuel_calc = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"
    .into();

  //println!("{:#?}", fuel_calc);
  assert_eq!(165, fuel_calc.calculate_ore_for_fuel(1));

  fuel_calc = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF 
177 ORE => 5 HKGWZ 
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF 
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
    .into();
  //println!("{:#?}", fuel_calc);
  assert_eq!(13312, fuel_calc.calculate_ore_for_fuel(1));
  assert_eq!(82892753, fuel_calc.calculate_fuel_for_1_trillion_ore());

  fuel_calc = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"
    .into();
  assert_eq!(180697, fuel_calc.calculate_ore_for_fuel(1));
  assert_eq!(5586022, fuel_calc.calculate_fuel_for_1_trillion_ore());

  fuel_calc = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"
    .into();
  assert_eq!(2210736, fuel_calc.calculate_ore_for_fuel(1));
  assert_eq!(460664, fuel_calc.calculate_fuel_for_1_trillion_ore());
}
