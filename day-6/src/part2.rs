
fn get_race(input: &str) -> (u64, u64) {

    let input = input.replace(' ', "");
    let mut iter = input.lines();

    let first = iter.next().unwrap().split_once(':').unwrap().1;
    let second = iter.next().unwrap().split_once(':').unwrap().1;

    (first.parse().unwrap(), second.parse().unwrap())

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    
    let race = get_race(input);
    let mut result: u64 = 0;

    (1..race.0).for_each(|time| {
        if (race.0 - time) * time > race.1 {
            result += 1;
        }
    });

    result.to_string()

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
