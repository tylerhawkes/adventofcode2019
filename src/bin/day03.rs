use std::ops::Add;
use std::str::FromStr;

fn main() {
  let paths = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day03.txt"));
  let paths: Vec<Vec<Path>> = paths
    .lines()
    .map(|s| {
      s.split(",")
        .map(|s| s.parse::<Path>().unwrap())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  //  println!("Paths: {:#?}", paths);

  let coords = paths
    .iter()
    .map(|v| {
      let mut coord = Coord { x: 0, y: 0 };
      std::iter::once(&Path::Up(0))
        .chain(v.iter())
        .map(|p| {
          coord = coord + *p;
          coord
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let segs = coords
    .iter()
    .map(|v| {
      v.windows(2)
        .map(|c| Segment {
          start: c[0],
          end: c[1],
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let intersections = segs[0]
    .iter()
    .cloned()
    .flat_map(|s1| {
      segs[1]
        .iter()
        .cloned()
        .filter_map(move |s2| s1.intersects(s2))
    })
    .collect::<Vec<_>>();

  let closest = intersections
    .iter()
    .cloned()
    .min_by_key(|c| c.manhattan_dist())
    .unwrap();
  println!(
    "closest: {:?} is {} away",
    closest,
    closest.manhattan_dist()
  );

  let segs_steps = segs
    .iter()
    .map(|v| {
      let mut seg_step = Vec::with_capacity(v.len());
      v.iter().fold(0, |steps, seg| {
        let steps = steps + seg.steps();
        seg_step.push((*seg, steps));
        steps
      });
      seg_step
    })
    .collect::<Vec<_>>();

  let intersection_steps = segs_steps[0]
    .iter()
    .cloned()
    .flat_map(|(seg1, steps1)| {
      segs_steps[1]
        .iter()
        .cloned()
        .filter_map(move |(seg2, steps2)| {
          if let Some(c) = seg1.intersects(seg2) {
            if c.x == seg1.start.x {
              let x_back = (seg2.end.x - c.x).abs() as u32;
              let y_back = (seg1.end.y - c.y).abs() as u32;
              Some((c, steps1 + steps2 - x_back - y_back))
            } else {
              let x_back = (seg1.end.x - c.x).abs() as u32;
              let y_back = (seg2.end.y - c.y).abs() as u32;
              Some((c, steps1 + steps2 - x_back - y_back))
            }
          } else {
            None
          }
        })
    })
    .collect::<Vec<_>>();

  let closest_steps = intersection_steps
    .iter()
    .cloned()
    .min_by_key(|c| c.1)
    .unwrap();
  println!(
    "closest steps: {:?} is {} away",
    closest_steps.0, closest_steps.1
  );
}

#[derive(Copy, Clone, Debug)]
enum Path {
  Up(i32),
  Down(i32),
  Left(i32),
  Right(i32),
}

impl FromStr for Path {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match &s[..1] {
      "U" => Self::Up(s[1..].parse().unwrap()),
      "D" => Self::Down(s[1..].parse().unwrap()),
      "L" => Self::Left(s[1..].parse().unwrap()),
      "R" => Self::Right(s[1..].parse().unwrap()),
      _ => unreachable!(),
    })
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Coord {
  x: i32,
  y: i32,
}

impl Coord {
  fn manhattan_dist(self) -> i32 {
    self.x.abs() + self.y.abs()
  }
}

impl Add<Path> for Coord {
  type Output = Coord;

  fn add(self, rhs: Path) -> Self::Output {
    match rhs {
      Path::Up(i) => Coord {
        x: self.x,
        y: self.y + i,
      },
      Path::Down(i) => Coord {
        x: self.x,
        y: self.y - i,
      },
      Path::Left(i) => Coord {
        x: self.x - i,
        y: self.y,
      },
      Path::Right(i) => Coord {
        x: self.x + i,
        y: self.y,
      },
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Segment {
  start: Coord,
  end: Coord,
}

impl Segment {
  fn xmin(self) -> i32 {
    self.start.x.min(self.end.x)
  }
  fn ymin(self) -> i32 {
    self.start.y.min(self.end.y)
  }
  fn xmax(self) -> i32 {
    self.start.x.max(self.end.x)
  }
  fn ymax(self) -> i32 {
    self.start.y.max(self.end.y)
  }
  fn is_vertical(self) -> bool {
    self.start.x == self.end.x
  }
  fn is_horizontal(self) -> bool {
    self.start.y == self.end.y
  }
  fn steps(self) -> u32 {
    let steps = if self.is_vertical() {
      self.ymax() - self.ymin()
    } else {
      self.xmax() - self.xmin()
    };
    steps as u32
  }
  fn intersects(self, other: Self) -> Option<Coord> {
    // Assuming wires don't run over each other
    if (self.is_horizontal() && other.is_horizontal())
      || (self.is_vertical() && other.is_vertical())
    {
      return None;
    }
    if self.is_vertical() {
      let self_between_other = other.xmin() <= self.start.x && self.start.x <= other.xmax();
      let other_between_self = self.ymin() <= other.start.y && other.start.y <= self.ymax();
      if self_between_other && other_between_self {
        Some(Coord {
          x: self.start.x,
          y: other.start.y,
        })
      } else {
        None
      }
    } else {
      let self_between_other = other.ymin() < self.start.y && self.start.y <= other.ymax();
      let other_between_self = self.xmin() < other.start.x && other.start.x <= self.xmax();
      if self_between_other && other_between_self {
        Some(Coord {
          x: other.start.x,
          y: self.start.y,
        })
      } else {
        None
      }
    }
  }
}

#[test]
fn test_intersects() {
  fn h_seg(x1: i32, x2: i32, y: i32) -> Segment {
    Segment {
      start: Coord { x: x1, y },
      end: Coord { x: x2, y },
    }
  }
  fn v_seg(y1: i32, y2: i32, x: i32) -> Segment {
    Segment {
      start: Coord { x, y: y1 },
      end: Coord { x, y: y2 },
    }
  }
  assert!(h_seg(0, 10, 5).is_horizontal());
  assert!(v_seg(0, 10, 5).is_vertical());
  assert_eq!(h_seg(0, 10, 0).intersects(h_seg(0, 10, 0)), None);
  assert_eq!(v_seg(0, 10, 0).intersects(v_seg(0, 10, 0)), None);
  assert_eq!(h_seg(0, 10, 0).intersects(v_seg(1, 11, 0)), None);
  assert_eq!(
    h_seg(-5, 5, 0).intersects(v_seg(-5, 5, 0)),
    Some(Coord { x: 0, y: 0 })
  );
  assert_eq!(
    v_seg(-5, 5, 0).intersects(h_seg(-5, 5, 0)),
    Some(Coord { x: 0, y: 0 })
  );
}
