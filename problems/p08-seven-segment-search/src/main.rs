#![feature(generators, generator_trait, iter_from_generator)]

use {
  anyhow::{
    anyhow,
    Error,
  },
  std::{
    collections::BTreeMap,
    io,
  },
};

mod parse {
  use {
    anyhow::{
      anyhow,
      Error,
    },
    bitflags::bitflags,
    nom::{
      self,
      bytes::complete::{
        tag,
        take_while,
      },
      combinator::map_res,
      multi::separated_list0,
      sequence::separated_pair,
      IResult,
    },
  };

  bitflags! {
    #[derive(Default)]
    pub (crate) struct Digits : u16 {
      const ZERO = 1 << 0;
      const ONE = 1 << 1;
      const TWO = 1 << 2;
      const THREE = 1 << 3;
      const FOUR = 1 << 4;
      const FIVE = 1 << 5;
      const SIX = 1 << 6;
      const SEVEN = 1 << 7;
      const EIGHT = 1 << 8;
      const NINE = 1 << 9;
    }
  }

  impl Into<u8> for Digits {
    fn into(self) -> u8 {
      let mut out = 0;
      if !(self & Digits::ZERO).is_empty() {
        out += 0;
      }
      if !(self & Digits::ONE).is_empty() {
        out += 1;
      }
      if !(self & Digits::TWO).is_empty() {
        out += 2;
      }
      if !(self & Digits::THREE).is_empty() {
        out += 3;
      }
      if !(self & Digits::FOUR).is_empty() {
        out += 4;
      }
      if !(self & Digits::FIVE).is_empty() {
        out += 5;
      }
      if !(self & Digits::SIX).is_empty() {
        out += 6;
      }
      if !(self & Digits::SEVEN).is_empty() {
        out += 7;
      }
      if !(self & Digits::EIGHT).is_empty() {
        out += 8;
      }
      if !(self & Digits::NINE).is_empty() {
        out += 9;
      }

      out
    }
  }

  bitflags! {
    #[derive(Default,)]
    pub (crate) struct Segments: u8 {
      const A = 1 << 0;
      const B = 1 << 1;
      const C = 1 << 2;
      const D = 1 << 3;
      const E = 1 << 4;
      const F = 1 << 5;
      const G = 1 << 6;
    }
  }

  impl Segments {
    pub fn all_iter() -> impl Iterator<Item = Segments> {
      std::iter::from_generator(|| {
        yield Segments::A;
        yield Segments::B;
        yield Segments::C;
        yield Segments::D;
        yield Segments::E;
        yield Segments::F;
        yield Segments::G;
      })
    }

    pub fn iter(&self) -> impl Iterator<Item = Segments> {
      let pred = *self;
      Segments::all_iter().filter(move |seg| (pred & *seg) != Segments::empty())
    }
  }

  #[derive(Debug)]
  pub(crate) struct InputLine {
    pub vals: [Segments; 10],
    pub output: [Segments; 4],
  }

  pub(crate) fn single_segment(chr: char) -> Result<Segments, Error> {
    match chr {
      'a' => Ok(Segments::A),
      'b' => Ok(Segments::B),
      'c' => Ok(Segments::C),
      'd' => Ok(Segments::D),
      'e' => Ok(Segments::E),
      'f' => Ok(Segments::F),
      'g' => Ok(Segments::G),
      _ => Err(anyhow!("Not a valid digit: {}", chr)),
    }
  }

  pub(crate) fn digit(chrs: &str) -> IResult<&str, Segments> {
    nom::multi::fold_many1(
      map_res(nom::character::complete::one_of("abcdefg"), single_segment),
      Segments::default,
      |a, b| -> Segments { a | b },
    )(chrs)
  }

  pub(crate) fn input_line(input: &str) -> IResult<&str, InputLine> {
    map_res(
      separated_pair(
        separated_list0(tag(" "), digit),
        tag(" | "),
        separated_list0(tag(" "), digit),
      ),
      |(left, right)| -> Result<InputLine, Error> {
        Ok(InputLine {
          vals: left.as_slice().try_into()?,
          output: right.as_slice().try_into()?,
        })
      },
    )(input)
  }
}

use parse::{
  Digits,
  InputLine,
  Segments,
};

fn main() -> Result<(), Error> {
  let lines = io::stdin()
    .lines()
    .map(|line| -> Result<parse::InputLine, Error> {
      Ok(
        parse::input_line(&line?)
          .map_err(|e| anyhow!("Parsing failed: {}", e))?
          .1,
      )
    })
    .collect::<Result<Vec<_>, Error>>()?;

  // Part 1
  {
    let mut counts: [i32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for line in lines.iter() {
      for digit in line.output {
        match digit.iter().count() {
          2 => counts[1] += 1,
          3 => counts[7] += 1,
          4 => counts[4] += 1,
          7 => counts[8] += 1,
          _ => {
            continue;
          }
        }
      }
    }
    dbg!(counts.iter().sum::<i32>());
  }

  // Part 2?
  {
    let mut counts: BTreeMap<Digits, u32> = Default::default();
    let result = lines.iter().map(|line| {
      let mut mapping = BTreeMap::<Segments, Digits>::default();
      let mut revmap = BTreeMap::<Digits, Segments>::default();
      for segments in line.vals.iter() {
        let entry = mapping.entry(*segments).or_insert(Digits::all());
        match segments.iter().count() {
          2 => {
            *entry = Digits::ONE;
            revmap.insert(Digits::ONE, *segments);
          }
          3 => {
            *entry = Digits::SEVEN;
            revmap.insert(Digits::SEVEN, *segments);
          }
          4 => {
            *entry = Digits::FOUR;
            revmap.insert(Digits::FOUR, *segments);
          }
          7 => {
            *entry = Digits::EIGHT;
            revmap.insert(Digits::EIGHT, *segments);
          }
          _ => {
            continue;
          }
        }
      }
      for segments in line.vals.iter() {
        let entry = mapping.entry(*segments).or_insert(Digits::all());
        match segments.iter().count() {
          5 if *segments & revmap[&Digits::ONE] == revmap[&Digits::ONE] => {
            *entry = Digits::THREE;
            revmap.insert(Digits::THREE, *segments);
          }
          5 if (*segments & revmap[&Digits::FOUR]).iter().count() == 2 => {
            *entry = Digits::TWO;
            revmap.insert(Digits::TWO, *segments);
          }
          5 => {
            *entry = Digits::FIVE;
            revmap.insert(Digits::FIVE, *segments);
          }
          6 if *segments & revmap[&Digits::FOUR] == revmap[&Digits::FOUR] => {
            *entry = Digits::NINE;
            revmap.insert(Digits::NINE, *segments);
          }
          6 if *segments & revmap[&Digits::ONE] == revmap[&Digits::ONE] => {
            *entry = Digits::ZERO;
            revmap.insert(Digits::ZERO, *segments);
          }
          6 => {
            *entry = Digits::SIX;
            revmap.insert(Digits::SIX, *segments);
          }
          2 | 3 | 4 | 7 => continue,
          _ => panic!("Unexpected number of segments set"),
        }
      }

      let out = line.output.iter()
        .rev()
        .map(|segments| -> u8 { mapping[segments].into() })
        .enumerate()
        .map(|(i, d)| d as usize * usize::pow(10, i as u32))
        .sum::<usize>();

      dbg!(out)
    }).sum::<usize>();
    dbg!(result);
  }

  Ok(())
}

mod tests {

  use {
    anyhow::Error,
    nom::{
      self,
      bytes::complete::{
        tag,
        take_while,
      },
      combinator::map_res,
      multi::separated_list0,
      sequence::separated_pair,
      IResult,
    },
  };

  #[test]
  fn parse_segment() {
    assert_eq!(
      crate::parse::single_segment('a').unwrap(),
      crate::parse::Segments::A
    )
  }

  #[test]
  fn parse_digit() {
    assert_eq!(
      crate::parse::digit("ab").unwrap(),
      ("", crate::parse::Segments::A | crate::parse::Segments::B)
    )
  }
}
