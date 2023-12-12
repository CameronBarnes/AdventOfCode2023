
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spring {
    Working,
    Broken,
    Unknown
}

#[derive(Debug)]
struct Data {
    vec: Vec<Spring>,
    check: Vec<u8>,
}

fn parse_line(line: &str) -> Data {
    let input = line.split_once(' ').unwrap();
    let vec = input.0.chars().map(|c| {
        if c == '.' {Spring::Working} else if c == '#' {Spring::Broken} else {Spring::Unknown}
    }).collect();
    let check = input.1.split(',').flat_map(str::parse::<u8>).collect();
    Data{vec, check}
}

fn get_possible_solutions(data: Data) -> usize {

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    input.lines().map(parse_line).map(get_possible_solutions).sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("???.### 1,1,3", "1")]
    #[case(".??..??...?##. 1,1,3", "4")]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", "1")]
    #[case("????.#...#... 4,1,1", "1")]
    #[case("????.######..#####. 1,6,5", "4")]
    #[case("?###???????? 3,2,1", "10")]
    #[case("#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1", "6")]
    #[case("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1", "21")]
    fn test_process(#[case] input: &str, #[case] result: &str) {
        assert_eq!(result.to_string(), process(input));
    }
}
