use std::collections::HashMap;
use std::str::FromStr;

fn main() {
  let orbit_relations = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day06.txt"));
  let orbits = orbit_relations.parse::<Orbits>().unwrap();
  let orbit_count = orbits.count_nodes();
  println!("orbit_count = {}", orbit_count);
  let to = orbits.transfer_orbits("YOU".into(), "SAN".into());
  println!("transfer_orbits = {}", to);
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Id {
  id: [u8; 3],
}

impl std::fmt::Debug for Id {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::str::from_utf8(&self.id[..]).unwrap().fmt(f)
  }
}

impl From<&str> for Id {
  fn from(s: &str) -> Self {
    let bytes = s.as_bytes();
    Self {
      id: [bytes[0], bytes[1], bytes[2]],
    }
  }
}

#[derive(Clone, Debug)]
struct Orbits {
  orbits: HashMap<Id, Id>,
  tree: HashMap<Id, Node>,
}

impl FromStr for Orbits {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut orbits = Self {
      orbits: HashMap::with_capacity(2000),
      tree: HashMap::new(),
    };
    s.lines().for_each(|s| {
      let mut split = s.split(")");
      let parent = Id::from(split.next().unwrap());
      let child = Id::from(split.next().unwrap());
      orbits.orbits.insert(child, parent);
    });
    Ok(orbits)
  }
}

impl Orbits {
  fn count_nodes(&self) -> usize {
    self
      .orbits
      .iter()
      .map(|(_, parent)| self.count_orbits(parent) + 1)
      .sum()
  }
  fn count_orbits(&self, id: &Id) -> usize {
    match self.orbits.get(id) {
      Some(parent) => self.count_orbits(parent) + 1,
      None => 0,
    }
  }
  fn orbit_path_to_com(&self, start: Id, mut ids: Vec<Id>) -> Vec<Id> {
    ids.push(start);
    if start == Id::from("COM") {
      ids
    } else {
      self.orbit_path_to_com(self.orbits.get(&start).cloned().unwrap(), ids)
    }
  }
  fn transfer_orbits(&self, start: Id, end: Id) -> usize {
    let mut start_path = self.orbit_path_to_com(start, Vec::new());
    let mut end_path = self.orbit_path_to_com(end, Vec::new());
    while start_path.last() == end_path.last() {
      start_path.pop();
      end_path.pop();
    }
    start_path.len() + end_path.len() - 2
  }
}

#[derive(Clone, Debug)]
struct Node {
  id: Id,
  parent: Id,
}
