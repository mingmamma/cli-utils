use cli_utils::colors::{ColorString, Color};

#[test]
fn test_red_color() {
    let mut color_string = ColorString {
        color: Color::Red,
        string: "Red".to_string(),
        colorised: "".to_string(),
    };
    color_string.paint();
    
    let expected_output = "\x1b[31mRed\x1b[0m";
    assert_eq!(color_string.colorised, expected_output);
}
