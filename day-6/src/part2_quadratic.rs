
fn get_race(input: &str) -> (u64, u64) {

    let input = input.replace(' ', "");
    let mut iter = input.lines();

    let first = iter.next().unwrap().split_once(':').unwrap().1;
    let second = iter.next().unwrap().split_once(':').unwrap().1;

    (first.parse().unwrap(), second.parse().unwrap())

}

fn calc_wins(time: u64, dist: u64) -> u64 {

    let ubound = (time as f64 + ((time as f64).powi(2) - 4.0 * (dist as f64 + 1.0)).sqrt()) / 2.0;
    (ubound.floor() - (time as f64 - ubound).ceil() + 1.0) as u64

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    
    let race = get_race(input);
    calc_wins(race.0, race.1).to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input));
    }
}
