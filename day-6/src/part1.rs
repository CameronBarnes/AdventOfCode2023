
fn get_races(input: &str) -> Vec<(i32, i32)> {
    let mut iter = input.lines();
    let mut first = iter.next().unwrap().split_whitespace();
    let mut second = iter.last().unwrap().split_whitespace();

    first.next();
    second.next();

    let mut result: Vec<(i32, i32)> = Vec::new();

    while let (Some(time), Some(distance)) = (first.next(), second.next())  {
        result.push((time.parse::<i32>().unwrap(), distance.parse::<i32>().unwrap()));
    }

    result

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    
    let races = get_races(input);
    let mut result = 0;

    for race in races {

        let mut count = 0;
        (1..race.0).for_each(|time| {
            if (race.0 - time) * time > race.1 {
                count += 1;
            }
        });

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
