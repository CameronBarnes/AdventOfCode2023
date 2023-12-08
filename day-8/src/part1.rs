use std::collections::HashMap;

fn get_path(str: &str) -> HashMap<String, (String, String)> {

    let mut map = HashMap::new();

    str.lines().for_each(|line| {
        let input = line.split_once(" = (").unwrap();
        let key = input.0.to_string();
        let value = input.1.split_once(", ").unwrap();
        let value_a = value.0.to_string();
        let value_b = value.1.strip_suffix(')').unwrap().to_string();
        map.insert(key, (value_a, value_b));
    });

    map

}

#[tracing::instrument]
pub fn process(input: &str) -> String {

    let input = input.split_once("\n\n").unwrap();
    let directions = input.0.trim();
    let path = get_path(input.1.trim());

    let mut steps = 0;
    let mut found = false;
    let mut current = "AAA".to_string();

    let mut directions_iter = directions.chars().cycle();
    while !found {

        steps += 1;
        match directions_iter.next().unwrap() {
            'L' => {
                current = path.get(&current).unwrap().0.clone();
            },
            'R' => {
                current = path.get(&current).unwrap().1.clone();
            },
            value => panic!("Unknown direction: {value}"),
        }

        if current.eq("ZZZ") {
            found = true;
        }

    }

    steps.to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input));
    }

    #[test]
    fn test_process2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input));
    }
}
