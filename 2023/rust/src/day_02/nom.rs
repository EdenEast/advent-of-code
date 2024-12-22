use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

use super::CubeSet;

impl CubeSet {
    fn from_color(color: &str, amount: u32) -> CubeSet {
        let mut set = CubeSet::default();
        match color {
            "red" => set.red = amount,
            "green" => set.green = amount,
            "blue" => set.blue = amount,
            _ => unreachable!(),
        }
        set
    }
}

impl<'a> std::ops::Add<&'a CubeSet> for CubeSet {
    type Output = CubeSet;

    fn add(self, rhs: &'a CubeSet) -> Self::Output {
        CubeSet {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

fn cube(input: &str) -> IResult<&str, CubeSet> {
    let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, CubeSet::from_color(color, amount)))
}

fn round(input: &str) -> IResult<&str, CubeSet> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    let set = cubes.iter().fold(CubeSet::default(), |acc, x| acc + x);
    Ok((input, set))
}

fn game(input: &str) -> IResult<&str, Vec<CubeSet>> {
    let (input, _) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((input, rounds))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
pub fn parse(input: &str) -> IResult<&str, Vec<Vec<CubeSet>>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}
