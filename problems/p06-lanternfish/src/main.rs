use {
  anyhow::{anyhow, Error},
  std::io::{self},
};

fn read_input(input: &str, round: usize) -> Result<[usize; 10], Error> {
  let mut counts: [usize; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

  for count in input.split(',').map(str::parse::<usize>) {
    counts[count?] += 1;
  }

  counts[0..7].rotate_right(round);

  Ok(counts)
}

fn step(counts: &mut [usize; 10], round: &mut usize) -> [usize; 10] {
  counts[9] = counts[*round % 7];
  counts[*round % 7] += counts[7];
  counts[7] = counts[8];
  counts[8] = counts[9];
  counts[9] = 0;
  *round += 1;
  *counts
}

fn main() -> Result<(), Error> {
  let mut counts = read_input(
    &io::stdin()
      .lines()
      .next()
      .ok_or(anyhow!("Failed to read input"))??,
      0
  )?;
  let mut round = 0;

  dbg!(counts);
  (0..256).for_each(|_| {step(&mut counts, &mut round);});

  dbg!(counts.iter().sum::<usize>());

  println!("round {} (offset {}): {:#?}", round, round % 7, counts);

  Ok(())
}

mod tests {
  #[test]
  fn sample() {
    let mut actual = crate::read_input("3,4,3,1,2", 0).unwrap();
    let mut round = 0;
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("2,3,2,0,1", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("1,2,1,6,0,8", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("0,1,0,5,6,7,8", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("6,0,6,4,5,6,7,8,8", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("5,6,5,3,4,5,6,7,7,8", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("4,5,4,2,3,4,5,6,6,7", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("3,4,3,1,2,3,4,5,5,6", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("2,3,2,0,1,2,3,4,4,5", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("1,2,1,6,0,1,2,3,3,4,8", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("0,1,0,5,6,0,1,2,2,3,7,8", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("6,0,6,4,5,6,0,1,1,2,6,7,8,8,8", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8", round % 7).unwrap() );
    assert_eq!( crate::step(&mut actual, &mut round), crate::read_input("6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8", round % 7).unwrap() );

    assert_eq!(actual.iter().sum::<usize>(), 26);
  }
}
