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

#[tracing::instrument]
pub fn process(input: &str) -> String {
    input.lines().map(|line| {
        let (_id, r, g, b) = parse_line(line);
        r * g * b
    }).sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input));
    }
}
