use {
  anyhow::{anyhow, Error},
  std::{
    io::{self, Read},
    str::FromStr,
  },
};

const BITS: usize = 12;

fn main() -> Result<(), Error> {
  let input = io::stdin()
    .lines()
    .map(|l| {
      l.map_err(Error::from)
        .and_then(|s| u64::from_str_radix(&s, 2).map_err(Error::from))
    })
    .collect::<Result<Vec<u64>, Error>>()?;
  
  // Part 1
  {
    // there's probably a better way to do this.
    let mut gamma: u64 = 0;
    for d in 0..BITS {
      let set = input
        .iter()
        .map(|i| (*i as usize & 1 << d) >> d)
        .sum::<usize>()
        > input.len() / 2;
      let bit: u64 = set.into();

      gamma |= bit << d;
    }

    let epsilon = !gamma & ((1 << BITS) - 1);

    dbg!(gamma);
    dbg!(epsilon);
    dbg!(gamma * epsilon);
  }

  // Part 2
  let p2 = |pred: fn(&[usize], &[usize]) -> bool| {
    let mut q = (0..input.len()).collect::<Vec<usize>>();
    for d in (0..BITS).rev() {
      if q.len() == 1 { break; }
      let mut set_idxs = Vec::<usize>::new();
      let mut unset_idxs = Vec::<usize>::new();
      for (idx, set) in q
        .into_iter()
        .map(|idx| (idx, (input[idx] & 1 << d) != 0))
        {
          if set { set_idxs.push(idx); }
          else { unset_idxs.push(idx); }
        }
      match pred(&set_idxs, &unset_idxs) {
        true => q = set_idxs,
        false => q = unset_idxs
      }
    }
    dbg!(input[*q.first().unwrap()])
  };

  dbg!(
    p2(|set, unset| set.len() >= unset.len()) *
    p2(|set, unset| unset.len() > set.len())
  );

  Ok(())
}