use num_integer::Integer;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn main() {
  let asteroids = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day10.txt"));
  let mut monitoring_station = asteroids.parse::<MonitoringStation>().unwrap();

  let most_visible = monitoring_station.most_visible();
  println!(
    "The most asteroids visible are {} at {:?}",
    most_visible.1, most_visible.0
  );
  let nth_vaporized = monitoring_station.nth_vaporized(most_visible.0, 200);
  println!(
    "nth_vaporized asteroid is at {:?} ({})",
    nth_vaporized,
    nth_vaporized.0 * 100 + nth_vaporized.1
  );
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Grid {
  Empty,
  Asteroid,
}

#[derive(Debug)]
struct MonitoringStation {
  asteroids: Vec<Vec<Grid>>,
}

impl MonitoringStation {
  fn most_visible(&self) -> ((usize, usize), usize) {
    let mut visible_count = vec![vec![0; self.asteroids[0].len()]; self.asteroids.len()];
    for y in 0..self.asteroids.len() {
      for x in 0..self.asteroids[0].len() {
        if self.asteroids[y][x] == Grid::Asteroid {
          //println!("Checking visibility for ({}, {})", x, y);
          let mut visible = HashSet::new();
          for y_candidate in 0..self.asteroids.len() {
            for x_candidate in 0..self.asteroids[0].len() {
              if self.asteroids[y_candidate][x_candidate] == Grid::Asteroid {
                visible.insert(Self::slope_to_other_asteroid(
                  x,
                  y,
                  x_candidate,
                  y_candidate,
                ));
              }
            }
          }
          visible_count[y][x] = visible.len() - 1;
        }
      }
    }
    let mut most_visible_coord = (0, 0);
    let mut most_visible = 0;
    for y in 0..self.asteroids.len() {
      for x in 0..self.asteroids[0].len() {
        if visible_count[y][x] > most_visible {
          most_visible = visible_count[y][x];
          most_visible_coord = (x, y);
        }
      }
    }
    (most_visible_coord, most_visible)
  }
  fn nth_vaporized(&mut self, (x, y): (usize, usize), n: usize) -> (usize, usize) {
    use Ordering::*;
    //key = slope, value = coordinates
    //Don't need to iterate since 200 is less than the number visible.
    let mut visible = HashMap::<(isize, isize), (usize, usize)>::new();
    for y_candidate in 0..self.asteroids.len() {
      for x_candidate in 0..self.asteroids[0].len() {
        if self.asteroids[y_candidate][x_candidate] == Grid::Asteroid {
          match visible.entry(Self::slope_to_other_asteroid(
            x,
            y,
            x_candidate,
            y_candidate,
          )) {
            Entry::Occupied(mut o) => {
              let current = o.get_mut();
              let current_dist = (((current.0 as isize - x as isize) as f64).powi(2)
                + ((current.1 as isize - y as isize) as f64).powi(2))
              .sqrt();
              let new_dist = (((x_candidate as isize - x as isize) as f64).powi(2)
                + ((y_candidate as isize - y as isize) as f64).powi(2))
              .sqrt();
              if new_dist < current_dist {
                *current = (x_candidate, y_candidate);
              }
            }
            Entry::Vacant(v) => {
              v.insert((x_candidate, y_candidate));
            }
          }
        }
      }
    }
    let mut visible = visible.into_iter().collect::<Vec<_>>();
    visible.sort_by_key(|((run, rise), _)| {
      // wish I knew how to do this quickly with geometry, but this will work...
      match (run.cmp(&0), rise.cmp(&0)) {
        (Equal, Less) => Sorting(0, 0.0),
        (Greater, Less) => Sorting(1, *run as f64 / rise.abs() as f64),
        (Greater, Equal) => Sorting(2, 0.0),
        (Greater, Greater) => Sorting(3, *rise as f64 / *run as f64),
        (Equal, Greater) => Sorting(4, 0.0),
        (Less, Greater) => Sorting(5, run.abs() as f64 / *rise as f64),
        (Less, Equal) => Sorting(6, 0.0),
        (Less, Less) => Sorting(7, rise.abs() as f64 / run.abs() as f64),
        (Equal, Equal) => Sorting(8, 0.0),
      }
    });
    visible[n - 1].1
  }
  fn slope_to_other_asteroid(
    x_current: usize,
    y_current: usize,
    x_candidate: usize,
    y_candidate: usize,
  ) -> (isize, isize) {
    let mut rise = y_candidate as isize - y_current as isize;
    let mut run = x_candidate as isize - x_current as isize;
    let mut gcd = rise.gcd(&run);
    while gcd > 1 {
      rise /= gcd;
      run /= gcd;
      gcd = rise.gcd(&run);
    }
    (run, rise)
  }
}

impl FromStr for MonitoringStation {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let asteroids = s
      .lines()
      .map(|l| {
        l.chars()
          .map(|c| {
            if c == '#' {
              Grid::Asteroid
            } else {
              Grid::Empty
            }
          })
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    Ok(Self { asteroids })
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Sorting(isize, f64);

impl PartialOrd for Sorting {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Sorting {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.0.cmp(&other.0) {
      Ordering::Equal => self.1.partial_cmp(&other.1).unwrap(),
      o => o,
    }
  }
}

impl Eq for Sorting {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_it() {
    let mut small = ".#..#
.....
#####
....#
...##"
      .parse::<MonitoringStation>()
      .unwrap();

    let most_visible = small.most_visible();
    println!("small: {:?}", small);
    assert_eq!(most_visible, 8);
  }

  #[test]
  fn test_slope_to_other_asteroid() {
    assert_eq!(
      MonitoringStation::slope_to_other_asteroid(0, 0, 4, 8),
      (-1, -2)
    );
    assert_eq!(
      MonitoringStation::slope_to_other_asteroid(0, 0, 4, 0),
      (-1, 0)
    );
  }
}
