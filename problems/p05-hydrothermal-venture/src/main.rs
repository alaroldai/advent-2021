use {
  anyhow::{anyhow, Error},
  itertools::Itertools,
  std::{
    collections::BTreeMap,
    io::{self, Read},
    str,
    ops::{Sub}
  },
};

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Default, Clone, Copy)]
struct Point {
  x: i64,
  y: i64,
}

impl Sub for Point {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
      Point {
        x: self.x - rhs.x,
        y: self.y - rhs.y,
      }
  }
}

impl Point {
  // Note that the cross product is always in the z direction, so we return it as i64.
  fn cross(self: Point, rhs: Point) -> i64 {
    self.x * rhs.y - rhs.x * self.y
  }

  fn dot(self: Point, rhs: Point) -> i64 {
    self.x * rhs.x + self.y * rhs.y
  }
}

#[derive(Debug, Clone, Copy)]
struct Line {
  start: Point,
  end: Point,
}

#[derive(PartialEq, Eq, Debug)]
enum Orientation {
  Horizontal,
  Vertical,
}

impl Line {
  fn contains(&self, point: Point) -> bool {
    match self.orientation() {
      Some(Orientation::Vertical) if self.start.x == point.x => {
        point.y >= i64::min(self.start.y, self.end.y)
          && point.y <= i64::max(self.start.y, self.end.y)
      }
      Some(Orientation::Horizontal) if self.start.y == point.y => {
        point.x >= i64::min(self.start.x, self.end.x)
          && point.x <= i64::max(self.start.x, self.end.x)
      }
      _ => false,
    }
  }

  fn contains2(&self, point: Point) -> bool {
    // If the line contains the point, then start -> point and point -> end are colinear

    let dot = (self.end - self.start).dot(point - self.start);

    let cross = (self.end - self.start).cross(point - self.start);

    cross == 0 && dot >= 0 && dot <= (self.end - self.start).dot(self.end - self.start)
  }

  fn orientation(&self) -> Option<Orientation> {
    if self.start.y == self.end.y {
      Some(Orientation::Horizontal)
    } else if self.start.x == self.end.x {
      Some(Orientation::Vertical)
    } else {
      None
    }
  }

  // fn intersects(&self, other: &Line) -> Option<Point> {
  //   let a1 = self.end.y - self.start.y;
  //   let b1 = self.start.x - self.end.y;
  //   let c1 = a1 * self.start.x + b1 * self.start.y;

  //   let a2 = other.end.y - other.start.y;
  //   let b2 = other.start.x - other.end.y;
  //   let c2 = a2 * other.start.x + b2 * other.start.y;

  //   let det = a1*b2 - a2 * b1;
  //   if det == 0 {
  //     None
  //   }
  //   else {
  //     let p = Point {
  //       x: (b2 * c1 - b1 * c2) / det,
  //       y: (a1 * c2 - a2 * c1) / det,
  //     };
  //     if p.x >= self.start.x && p.x <= self.end.x && p.x >= other.start.x && p.x <= other.end.x {
  //       Some(p)
  //     }
  //     else {
  //       None
  //     }
  //   }
  // }
}

mod parse {
  use {
    super::*,
    nom::{
      bytes::complete::{tag, take_while},
      combinator::map_res,
      multi::separated_list0,
      sequence::separated_pair,
      IResult,
    },
  };

  fn is_dec_digit(c: char) -> bool {
    c.is_ascii_digit()
  }

  fn dec_value(input: &str) -> IResult<&str, i64> {
    map_res(take_while(is_dec_digit), |s: &str| s.parse::<i64>())(input)
  }

  fn point(i: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(dec_value, tag(","), dec_value)(i)?;

    Ok((input, Point { x, y }))
  }

  fn line(i: &str) -> IResult<&str, Line> {
    let (input, (start, end)) = separated_pair(point, tag(" -> "), point)(i)?;

    Ok((input, Line { start, end }))
  }

  pub(crate) fn input(i: &str) -> IResult<&str, Vec<Line>> {
    separated_list0(tag("\n"), line)(i)
  }
} // mod parse

fn main() -> Result<(), Error> {
  let mut input_str = String::new();
  io::stdin().read_to_string(&mut input_str)?;

  let Ok((_, lines)) = parse::input(&input_str) else {
    return Err(anyhow!("Parsing failed"));
  };

  // naive: compute a bounding box for the area, then iterate over all the points and find lines that intersect the point.
  {
    let mut min: Point = Point {
      x: i64::max_value(),
      y: i64::max_value(),
    };
    let mut max: Point = Default::default();
    // Bounding box
    for line in lines.iter() {
      min.x = i64::min(min.x, i64::min(line.start.x, line.end.x));
      max.x = i64::max(max.x, i64::max(line.start.x, line.end.x));
      min.y = i64::min(min.y, i64::min(line.start.y, line.end.y));
      max.y = i64::max(max.y, i64::max(line.start.y, line.end.y));
    }

    let mut count_part1 = 0;
    let mut count_part2 = 0;
    for (x, y) in (min.x..=max.x).cartesian_product(min.y..=max.y) {
      // part 1
      {
        let intersections = lines
          .iter()
          .filter(|line| line.contains(Point { x, y }))
          .count();
        // println!("{:?} found {} intersections", Point { x, y }, intersections);
        if intersections > 1 {
          count_part1 += 1;
        }
      }
      
      dbg!(count_part1);

      // part 2
      {
        let intersections = lines
          .iter()
          .filter(|line| line.contains2(Point { x, y }))
          .count();
        // println!("{:?} found {} intersections", Point { x, y }, intersections);
        if intersections > 1 {
          count_part2 += 1;
        }
      }
    }

    dbg!(count_part2);
  }

  // TODO: Build range lists for each row and column.
  // i.e. for each row and column:
  // - Collect lines in that row/col
  // - Sort by start and end to make finding intersections easy.

  // These map row/column position to vector of indices into `lines` for
  // let mut rows: BTreeMap<i64, Vec<usize> = BTreeMap::new();
  // let mut cols: BTreeMap<i64, Vec<usize> = BTreeMap::new();

  Ok(())
}

mod tests {

  #[test]
  fn test_contains_horizontal() {
    use super::{Line, Point};
    let line = Line {
      start: Point { x: 0, y: 9 },
      end: Point { x: 5, y: 9 },
    };
    let point = Point { x: 2, y: 9 };

    assert_eq!(line.orientation(), Some(crate::Orientation::Horizontal));

    assert_eq!(line.start.y, point.y);
    assert!(point.y >= i64::min(line.start.y, line.end.y));
    assert!(point.y <= i64::max(line.start.y, line.end.y));

    assert!(line.contains(point));
  }

  #[test]
  fn test_contains_vertical() {
    use super::{Line, Point};

    let line = Line {
      start: Point { x: 5, y: 0 },
      end: Point { x: 5, y: 9 },
    };

    assert_eq!(line.orientation(), Some(crate::Orientation::Vertical));

    assert!(line.contains(Point { x: 5, y: 4 }));
  }

  #[test]
  fn test_contains2() {
    use super::{Line, Point};

    let line = Line {
      start: Point { x: 1, y: 1 },
      end: Point { x: 3, y: 3 },
    };

    assert_eq!(line.orientation(), None);

    assert!(line.contains2(Point { x: 2, y: 2 }));
    assert!(!line.contains2(Point { x: 4, y: 4 }));

    let line = Line {
      start: Point { x: 9, y: 7 },
      end: Point { x: 7, y: 9 },
    };

    assert_eq!(line.orientation(), None);

    assert!(line.contains2(Point { x: 8, y: 8 }));
    assert!(!line.contains2(Point { x: 6, y: 10 }));
  }

  #[test]
  fn test_orientation() {
    use super::{Line, Orientation, Point};

    assert_eq!(
      Line {
        start: Point { x: 0, y: 9 },
        end: Point { x: 5, y: 9 }
      }
      .orientation(),
      Some(Orientation::Horizontal)
    );

    assert_eq!(
      Line {
        start: Point { x: 0, y: 0 },
        end: Point { x: 0, y: 9 }
      }
      .orientation(),
      Some(Orientation::Vertical)
    );
  }
}
