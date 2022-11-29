use {
  anyhow::{anyhow, Error},
  std::{
    fmt, io,
    ops::{Index, IndexMut},
    str::FromStr,
  },
};

#[derive(Default)]
struct BingoBoard {
  nums: [u8; 25],
  rows: [u8; 5],
  cols: [u8; 5],
}

impl fmt::Debug for BingoBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for x in 0..5 {
      for y in 0..5 {
        write!(f, "{:3} ", self[(x, y)])?;
      }
      writeln!(f)?;
    }
    writeln!(f, "{:?}", self.rows)?;
    writeln!(f, "{:?}", self.cols)?;
    Ok(())
  }
}

impl BingoBoard {
  fn mark(&mut self, val: u8) {
    for x in 0..5 {
      for y in 0..5 {
        if self[(x, y)] == val {
          self.rows[x] |= 1 << y;
          self.cols[y] |= 1 << x;
        }
      }
    }
  }

  fn reset(&mut self) {
    for x in 0..5 { self.rows[x] = 0; self.cols[x] = 0; }
  }

  fn score(&self) -> u32 {
    let mut sum = 0u32;
    for x in 0..5 {
      for y in 0..5 {
        if !self.rows[x] & (1 << y) > 0 {
          sum += self[(x, y)] as u32;
        }
      }
    }

    sum
  }

  fn finished(&self) -> bool {
    for r in self.rows.iter().chain(self.cols.iter()) {
      if *r == (1 << 5) - 1 {
        return true;
      }
    }
    false
  }
}

impl Index<(usize, usize)> for BingoBoard {
  type Output = u8;

  fn index(&self, index: (usize, usize)) -> &Self::Output {
    let (x, y) = index;
    &self.nums[x * 5 + y]
  }
}

impl IndexMut<(usize, usize)> for BingoBoard {
  fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
    let (x, y) = index;
    &mut self.nums[x * 5 + y]
  }
}

impl FromStr for BingoBoard {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut out = BingoBoard::default();

    for (i, line) in s.lines().enumerate() {
      for (j, c) in line.split_whitespace().enumerate() {
        out[(i, j)] = c.parse()?;
      }
    }

    Ok(out)
  }
}

pub trait DiscardInput {
  fn discard_line(&mut self) -> Result<bool, io::Error>;
}

impl DiscardInput for io::Stdin {
  fn discard_line(&mut self) -> Result<bool, io::Error> {
    use std::io::BufRead;
    let mut buf = self.lock();
    loop {
      let (consume, done) = {
        let data = buf.fill_buf()?;
        if data.is_empty() {
          return Ok(false);
        } else if let Some(nl) = data.iter().position(|b| *b == b'\n') {
          (nl + 1, true)
        } else {
          (data.len(), false)
        }
      };
      buf.consume(consume);
      if done {
        break;
      }
    }
    Ok(true)
  }
}

fn main() -> Result<(), Error> {
  // use std::collections::BTreeMap;

  let input = io::stdin()
    .lines()
    .take(1)
    .next()
    .ok_or_else(|| anyhow!("Ran out of input"))??
    .split(',')
    .map(|s| s.parse().map_err(Error::from))
    .collect::<Result<Vec<u8>, Error>>()?;

  let mut boards: Vec<BingoBoard> = Vec::new();

  loop {
    if !io::stdin().discard_line()? {
      break;
    }
    let mut board_in = String::new();
    for _ in 0..5 {
      io::stdin().read_line(&mut board_in)?;
    }
    let board: BingoBoard = board_in.parse()?;
    boards.push(board);
  }

  // dbg!(&boards);

  // let mut index: BTreeMap<u8, Vec<usize>> = BTreeMap::new();
  // for (i, board) in boards.iter().enumerate() {
  //   for n in board.nums {
  //     index.entry(n).or_default().push(i)
  //   }
  // }

  // Part 1
  {
    let mut winner = None;
    let mut val = None;
    'outer: for n in input.iter() {
      for (i, board) in boards.iter_mut().enumerate() {
        board.mark(*n);
        if board.finished() {
          println!("Board {} finished", i);
          winner = Some(i);
          val = Some(*n as u32);
          break 'outer;
        }
      }
    }
    // dbg!(&boards);

    let (Some(winner), Some(val)) = (winner, val) else {
    return Err(anyhow!("??"));
  };

    dbg!(boards[winner].score() * val);
  }

  for board in boards.iter_mut() { board.reset(); }

  // Part 2
  {
    let mut val = None;
    for n in input {
      dbg!(boards.len());
      for board in boards.iter_mut() {
        board.mark(n);
      }
      if boards.len() == 1 && boards[0].finished() {
        val = Some(n as u32);
        break;
      }
      boards.retain(|b| !b.finished());
    }
    let (winner, Some(val)) = (&boards[0], val) else {
      return Err(anyhow!("??"));
    };
    dbg!(winner.score() * val);
    dbg!(winner);
  }

  Ok(())
}
