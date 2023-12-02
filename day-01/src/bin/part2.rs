

fn parse_line(str: &str) -> i32 {

    let str = str.replace("one", "o1e");
    let str = str.replace("two", "t2o");
    let str = str.replace("three", "t3e");
    let str = str.replace("four", "f4r");
    let str = str.replace("five", "f5e");
    let str = str.replace("six", "s6x");
    let str = str.replace("seven", "s7n");
    let str = str.replace("eight", "e8t");
    let str = str.replace("nine", "n9e");

    let matches : Vec<&str> = str.matches(char::is_numeric).collect();
    let mut out = matches[0].to_string();
    out.push_str(matches[matches.len() - 1]);

    out.parse().unwrap()

}

fn main() {

    let input = include_str!("./input.txt");
    let result: i32 = input.lines().map(parse_line).sum();
    println!("Day 1 Part 2: {result}");

}
