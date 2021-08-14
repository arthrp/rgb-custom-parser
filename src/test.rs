use super::*;

#[test]
fn parses_basic_strings() {
    let (_, c) = rgb_color("rgb(100,200,255)").unwrap();

    assert_eq!(c.red, 100);
    assert_eq!(c.blue, 255);
    assert_eq!(c.green, 200);

    let (_, c1) = rgb_color("rgb(0,0,200)").unwrap();

    assert_eq!(c1.red, 0);
    assert_eq!(c1.blue, 200);
    assert_eq!(c1.green, 0);
}

#[test]
fn parses_strings_with_whitespaces() {
    let (_, c) = rgb_color("rgb(100, 200, 255)").unwrap();

    assert_eq!(c.red, 100);
    assert_eq!(c.blue, 255);
    assert_eq!(c.green, 200);

    let (_, c1) = rgb_color("rgb(0,    0,   200)").unwrap();

    assert_eq!(c1.red, 0);
    assert_eq!(c1.blue, 200);
    assert_eq!(c1.green, 0);
}