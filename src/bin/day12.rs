use std::cmp::Ordering;
use std::ops::Add;
use std::str::FromStr;
use std::time::Instant;

fn main() {
  let start = Instant::now();
  let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day12.txt"));
  let mut moons = input.parse::<Moons>().unwrap();

  {
    let mut moons = moons.clone();
    for _ in 0..1000 {
      moons.step();
    }

    let total_energy = moons.moons.iter().map(|m| m.total_energy()).sum::<usize>();
    println!("Total energy: {}", total_energy);
  }

  println!("Repeats after: {}", moons.repeats_after());
  println!("Took {:?}", start.elapsed());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
  x: i32,
  y: i32,
  z: i32,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Velocity {
  x: i32,
  y: i32,
  z: i32,
}

impl Coord {
  fn gravity(self, other: Self) -> Velocity {
    Velocity {
      x: Self::gravity_adjustment(self.x, other.x),
      y: Self::gravity_adjustment(self.y, other.y),
      z: Self::gravity_adjustment(self.z, other.z),
    }
  }
  fn gravity_adjustment(x: i32, y: i32) -> i32 {
    match x.cmp(&y) {
      Ordering::Equal => 0,
      Ordering::Less => 1,
      Ordering::Greater => -1,
    }
  }
  fn potential_energy(self) -> usize {
    (self.x.abs() + self.y.abs() + self.z.abs()) as usize
  }
}

impl Velocity {
  fn kinetic_energy(self) -> usize {
    (self.x.abs() + self.y.abs() + self.z.abs()) as usize
  }
}

impl Add<Velocity> for Coord {
  type Output = Self;

  fn add(self, rhs: Velocity) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl Add for Velocity {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

#[derive(Clone, Debug)]
struct Moon {
  position: Coord,
  velocity: Velocity,
  new_velocity: Velocity,
}

impl Moon {
  fn total_energy(&self) -> usize {
    self.position.potential_energy() * self.velocity.kinetic_energy()
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct State {
  position: Coord,
  velocity: Velocity,
}

impl FromStr for Moon {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    //<x=2, y=19, z=15>
    let s = s.replace("<", "").replace(">", "");
    let p = s
      .split(",")
      .map(|s| s.split("=").skip(1).next().unwrap())
      .map(|s| s.parse::<i32>().unwrap())
      .collect::<Vec<_>>();
    Ok(Self {
      position: Coord {
        x: p[0],
        y: p[1],
        z: p[2],
      },
      velocity: Velocity::default(),
      new_velocity: Velocity::default(),
    })
  }
}

#[derive(Clone, Debug)]
struct Moons {
  moons: Vec<Moon>,
  initial: Vec<Moon>,
}

impl FromStr for Moons {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let moons = s
      .lines()
      .map(|s| s.parse::<Moon>().unwrap())
      .collect::<Vec<_>>();
    Ok(Self {
      initial: moons.clone(),
      moons,
    })
  }
}

impl Moons {
  fn apply_gravity(&mut self) {
    self
      .moons
      .iter_mut()
      .for_each(|m| m.new_velocity = Velocity::default());
    for x in 0..self.moons.len() {
      for y in 0..self.moons.len() {
        if x != y {
          self.moons[x].new_velocity =
            self.moons[x].new_velocity + self.moons[x].position.gravity(self.moons[y].position)
        }
      }
    }
  }

  fn apply_velocity(&mut self) {
    self.moons.iter_mut().for_each(|m| {
      m.velocity = m.velocity + m.new_velocity;
      m.new_velocity = Velocity::default();
      m.position = m.position + m.velocity;
    })
  }

  fn step(&mut self) {
    self.apply_gravity();
    self.apply_velocity();
  }

  fn repeats_after(&mut self) -> u128 {
    use num_integer::Integer;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    for i in 1..1_000_000_000_u128 {
      self.step();
      if x == 0
        && self
          .moons
          .iter()
          .zip(self.initial.iter())
          .all(|(m, i)| m.position.x == i.position.x && m.velocity.x == i.velocity.x)
      {
        x = i;
      }
      if y == 0
        && self
          .moons
          .iter()
          .zip(self.initial.iter())
          .all(|(m, i)| m.position.y == i.position.y && m.velocity.y == i.velocity.y)
      {
        y = i;
      }
      if z == 0
        && self
          .moons
          .iter()
          .zip(self.initial.iter())
          .all(|(m, i)| m.position.z == i.position.z && m.velocity.z == i.velocity.z)
      {
        z = i;
      }
      if x > 0 && y > 0 && z > 0 {
        break;
      }
    }
    x.lcm(&y).lcm(&z)
  }
}
