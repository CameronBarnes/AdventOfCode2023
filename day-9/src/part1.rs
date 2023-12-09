
fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace().flat_map(str::parse::<i32>).collect()
}

fn extrapolate(values: Vec<i32>) -> i32 {

    if values.iter().all(|val| *val == 0) {
        return 0;
    }

    let mut ex_1 = Vec::with_capacity(values.len() - 1);

    for i in 0..values.len() - 1 {
        ex_1.push(values[i + 1] - values[i]);
    }

    values[values.len() - 1] + extrapolate(ex_1)

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    input.lines().map(parse_line).map(extrapolate).sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("0 3 6 9 12 15", 18)]
    #[case("1 3 6 10 15 21", 28)]
    #[case("10 13 16 21 30 45", 68)]
    fn test_extrapolate(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(expected, extrapolate(parse_line(input)));
    } 

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("114", process(input));
    }
}
