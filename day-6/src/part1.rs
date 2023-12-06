
fn get_races(input: &str) -> Vec<(u32, u32)> {
    let mut iter = input.lines();
    let mut first = iter.next().unwrap().split_whitespace();
    let mut second = iter.last().unwrap().split_whitespace();

    first.next();
    second.next();

    let mut result: Vec<(u32, u32)> = Vec::new();

    while let (Some(time), Some(distance)) = (first.next(), second.next())  {
        result.push((time.parse::<u32>().unwrap(), distance.parse::<u32>().unwrap()));
    }

    result

}

fn calc_wins(time: u32, dist: u32) -> u32 {

    for press in 1..time {
        if (time - press) * press > dist {
            return (time - press * 2) + 1;
        }
    }

    0

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    
    let races = get_races(input);
    let mut result = 0;

    for race in races {

        let count = calc_wins(race.0, race.1);

        if result == 0 {
            result = count;
        } else {
            result *= count;
        }

    }

    result.to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input));
    }
}
