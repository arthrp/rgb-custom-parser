extern crate nom;

use nom::sequence::tuple;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
use std::num::ParseIntError;
use nom::IResult;
use nom::bytes::complete::tag;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub struct ColorRgb {
    red : u8,
    green: u8,
    blue: u8
}

fn is_digit(c: char) -> bool {
    return c.is_digit(10);
}

fn num_to_int(str: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(str, 10)
}

fn color_number(input: &str) -> IResult<&str, u8> {
    let (input, num) = take_while1(is_digit)(input)?;

    let converted = num_to_int(num).unwrap();
    return Ok((input, converted));
}

fn rgb_color(input: &str) -> IResult<&str, ColorRgb> {
    let (input,_) = tag("rgb(")(input)?;
    let (input, (red, _, green, _, blue)) = tuple((color_number, separator, color_number, separator, color_number))(input)?;

    return Ok((input, ColorRgb { red, green, blue }));
}

fn separator(input: &str) -> IResult<&str, &str> {
    let (input, comma) = tag(",")(input)?;
    let (input, _) = take_while(|c| c == ' ')(input)?;

    return Ok((input, comma));
}

fn main() {
    let (_, color) = rgb_color("rgb(200,   255, 100)").unwrap();

    println!("Color is: {} {} {}", color.red, color.green, color.blue);
}

