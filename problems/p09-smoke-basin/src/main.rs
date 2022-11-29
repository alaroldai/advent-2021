use {
  anyhow::{
    anyhow,
    Error,
  },
  std::{
    io,
    ops,
    collections::{BTreeSet, VecDeque}
  },
};

#[derive(Default)]
struct Heightmap {
  width: usize,
  height: usize,
  map: Vec<u8>,
}

impl Heightmap {
  fn from_stdin() -> Result<Heightmap, Error> {
    let mut result = Heightmap::default();
    for line in io::stdin().lines() {
      let line = line?;
      result.width = line.len();
      result.height += 1;
      result.map.extend(line.bytes().map(|b| {
        match b {
          b'0' => 0,
          b'1' => 1,
          b'2' => 2,
          b'3' => 3,
          b'4' => 4,
          b'5' => 5,
          b'6' => 6,
          b'7' => 7,
          b'8' => 8,
          b'9' => 9,
          _ => panic!("Unexpected input"),
        }
      }));
    }

    Ok(result)
  }
}

impl ops::Index<(usize, usize)> for Heightmap {
  type Output = u8;

  fn index(&self, (x, y): (usize, usize)) -> &Self::Output { &self.map[x + y * self.width] }
}

impl ops::IndexMut<(usize, usize)> for Heightmap {
  fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
    &mut self.map[x + y * self.width]
  }
}

fn main() -> Result<(), Error> {
  let map = Heightmap::from_stdin()?;

  dbg!(map.width);
  dbg!(map.height);

  // Part 1
  {
    let mut risk_sum = 0usize;
    for y in 0..map.height {
      for x in 0..map.width {
        if x > 0 && map[(x - 1, y)] <= map[(x, y)] {
          continue;
        }
        if y > 0 && map[(x, y - 1)] <= map[(x, y)] {
          continue;
        }
        if x < map.width - 1 && map[(x + 1, y)] <= map[(x, y)] {
          continue;
        }
        if y < map.height - 1 && map[(x, y + 1)] <= map[(x, y)] {
          continue;
        }
        risk_sum += map[(x, y)] as usize + 1;
      }
    }

    dbg!(risk_sum);
  }

  // Part 2
  {
    let mut sizes = [0, 0, 0, 0];
    for y in 0..map.height {
      for x in 0..map.width {
        if x > 0 && map[(x - 1, y)] <= map[(x, y)] {
          continue;
        }
        if y > 0 && map[(x, y - 1)] <= map[(x, y)] {
          continue;
        }
        if x < map.width - 1 && map[(x + 1, y)] <= map[(x, y)] {
          continue;
        }
        if y < map.height - 1 && map[(x, y + 1)] <= map[(x, y)] {
          continue;
        }
        
        // We found a low point
        // Begin searching outward
        let mut visited: BTreeSet<(usize, usize)> = Default::default();
        let mut q = VecDeque::from([(x, y)]);
        let mut size = 0;
        while let Some((x, y)) = q.pop_front() {
          if visited.contains(&(x, y)) {continue};
          visited.insert((x, y));
          if map[(x, y)] >= 9 { continue; }
          if x > 0 { q.push_back((x-1, y)); }
          if y > 0 { q.push_back((x, y-1)); }
          if x < map.width-1 { q.push_back((x+1, y)); }
          if y < map.height-1 { q.push_back((x, y+1)); }
          size += 1;
        }
        sizes[3] = size;
        sizes.sort();
        sizes.reverse();
      }
    }
    dbg!(&sizes[0..3].iter().product::<i32>());
  }

  Ok(())
}
