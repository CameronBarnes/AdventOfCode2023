
fn get_races(input: &str) -> Vec<(u64, u64)> {
    let mut iter = input.lines();
    let mut first = iter.next().unwrap().split_whitespace();
    let mut second = iter.last().unwrap().split_whitespace();

    first.next();
    second.next();

    let mut result: Vec<(u64, u64)> = Vec::new();

    while let (Some(time), Some(distance)) = (first.next(), second.next())  {
        result.push((time.parse::<u64>().unwrap(), distance.parse::<u64>().unwrap()));
    }

    result

}

fn calc_wins(time: u64, dist: u64) -> u64 {

    let ubound = (time as f64 + ((time as f64).powi(2) - 4.0 * (dist as f64 + 1.0)).sqrt()) / 2.0;
    (ubound.floor() - (time as f64 - ubound).ceil() + 1.0) as u64

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
