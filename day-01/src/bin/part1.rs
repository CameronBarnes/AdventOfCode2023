

fn parse_line(str: &str) -> i32 {

    let matches : Vec<&str> = str.matches(char::is_numeric).collect();
    let mut out = matches[0].to_string();
    out.push_str(matches[matches.len() - 1]);

    out.parse().unwrap()

}

fn main() {

    let input = include_str!("./input.txt");
    let result: i32 = input.lines().map(parse_line).sum();
    println!("{result}");

}
