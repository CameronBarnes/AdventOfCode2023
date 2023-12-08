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

fn get_starting_positions(path: &HashMap<String, (String, String)>) -> Vec<String> {
    path.keys().filter(|key| key.ends_with('A')).cloned().collect()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn findlcm(arr: Vec<usize>) -> usize {
    // Initialize result
    let mut ans = arr[0];
 
    // ans contains LCM of arr[0], ..arr[i]
    // after i'th iteration,
    (1..arr.len()).for_each(|i| {
        ans = (arr[i] * ans) /
                gcd(arr[i], ans);
    });
 
    ans
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let input = input.split_once("\n\n").unwrap();
    let directions = input.0.trim();
    let path = get_path(input.1.trim());

    let mut steps = 0;
    let mut found = false;
    let mut current = get_starting_positions(&path);

    let mut lengths: HashMap<String, usize> = HashMap::new();

    let mut directions_iter = directions.chars().cycle();
    while !found {

        steps += 1;
        let dir = directions_iter.next().expect("Is cycling so it will always have a next value");
        (0..current.len()).for_each(|i| {
            match dir {
                'L' => {
                    current[i] = path.get(&current[i]).unwrap().0.clone();
                },
                'R' => {
                    current[i] = path.get(&current[i]).unwrap().1.clone();
                },
                value => panic!("Invalid direction: {value}"),
            }
        });

        for loc in &current {
            if loc.ends_with('Z') {
                lengths.entry(loc.to_string()).or_insert(steps);
            }
        }

        if lengths.len() == current.len() {
            found = true;
        }

    }

    findlcm(lengths.values().cloned().collect::<Vec<usize>>()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input));
    }
}
