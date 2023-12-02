fn parse_line(str: &str) -> (i32, i32, i32, i32) {

    let mut iter = str.split(':');
    let id = iter.next().unwrap().split(' ').last().unwrap().parse::<i32>().unwrap();
    let games_line = iter.next().unwrap();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for blocks in games_line.split(';') {
        for block in blocks.split(',') {
            let mut iter = block.trim().split(' ');
            let num = iter.next().unwrap().parse::<i32>().unwrap();
            match iter.next().unwrap() {
                "red" => red = i32::max(red, num),
                "green" => green = i32::max(green, num),
                "blue" => blue = i32::max(blue, num),
                _ => panic!("invalid colour")
            }
        }
    }

    (id, red, green, blue)

}

fn process(input: &str) -> String {

    input.lines().map(|line| {
        let (id, r, g, b) = parse_line(line);
        if r <= 12 && g <= 13 && b <= 14 {
            id
        } else {
            0
        }
    }).sum::<i32>().to_string()

}

fn main() {

    let input = include_str!("./input.txt");
    let result = process(input);
    println!("Day 2 Part 1: {result}");

}
