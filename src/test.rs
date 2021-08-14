use super::*;

#[test]
fn parses_basic_cases(){
    let (_, c) = rgb_color("rgb(100,200,255)").unwrap();

    assert_eq!(c.red, 100);
    assert_eq!(c.blue, 255);
    assert_eq!(c.green, 200);
}