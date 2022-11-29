use {
  anyhow::Result,
  std::{
    io::{self, Read},
    num::ParseIntError,
    str::FromStr,
  },
};

struct InputFile {
  depths: Vec<u16>,
}

impl FromStr for InputFile {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, ParseIntError> {
    Ok(InputFile {
      depths: s
        .lines()
        .map(|s| s.parse())
        .collect::<Result<Vec<u16>, ParseIntError>>()?,
    })
  }
}

fn main() -> anyhow::Result<()> {
  // Problem 1: read the input, then count the number of times the depth
  // increases from one measurement to the next
  let mut stdin = String::new();
  io::stdin().read_to_string(&mut stdin)?;

  let input: InputFile = stdin.parse()?;

  println!(
    "part 1: {}",
    input
      .depths .iter() .skip(1)
      .zip(input.depths.iter())
      .map(|(a, b)| -> u16 { (a > b).into() })
      .sum::<u16>()
  );

  println!(
    "part 2: {}",
    input
      .depths.windows(3).skip(1)
      .zip(input.depths.windows(3))
      .map(|(a, b)| -> u16 { (a.iter().sum::<u16>() > b.iter().sum()).into() })
      .sum::<u16>()
  );

  Ok(())
}
