use {
  anyhow::{anyhow, Result},
  std::{
    io::{self, Read},
    str::FromStr,
  },
};

enum Direction {
  Forward,
  Down,
  Up,
}

impl FromStr for Direction {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "forward" => Ok(Direction::Forward),
      "down" => Ok(Direction::Down),
      "up" => Ok(Direction::Up),
      other => Err(anyhow!("Couldn't parse direction from {}", other)),
    }
  }
}

struct Move {
  dir: Direction,
  dist: u32,
}

impl FromStr for Move {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut wsit = s.split_whitespace();

    Ok(Move {
      dir: wsit
        .next()
        .ok_or_else(|| anyhow!("No direction"))?
        .parse()?,
      dist: wsit.next().ok_or_else(|| anyhow!("No distance"))?.parse()?,
    })
  }
}

struct InputFile {
  moves: Vec<Move>,
}

impl FromStr for InputFile {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self> {
    Ok(InputFile {
      moves: s.lines().map(Move::from_str).collect::<Result<Vec<Move>>>()?,
    })
  }
}

fn main() -> Result<()> {
  let mut stdin = String::new();
  io::stdin().read_to_string(&mut stdin)?;

  let (mut x, mut y) = (0, 0);
  let mut aim: u32;

  let input: InputFile = stdin.parse()?;

  for mv in &input.moves {
    match mv.dir {
      Direction::Forward => x += mv.dist,
      Direction::Up => y -= mv.dist,
      Direction::Down => y += mv.dist,
    }
  }

  println!("Part 1: {}", x * y);

  (x, y, aim) = (0, 0, 0);

  for mv in &input.moves {
    match mv.dir {
      Direction::Forward => { x += mv.dist; y += mv.dist * aim; },
      Direction::Up => aim -= mv.dist,
      Direction::Down => aim += mv.dist,
    }
  }

  println!("Part 2: {}", x * y);

  Ok(())
}
