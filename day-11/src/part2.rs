use std::collections::HashSet;

fn to_char_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn char_map_to_expanded_pos(map: &[Vec<char>], mult: usize) -> Vec<(usize, usize)> {

    let mut locations = Vec::new();

    let mut x_offset: usize = 0;

    for x in 0..map[0].len() {

        let found = true;
        for y in 0..map.len() {

            let mut y_offset: usize = 0;
            (0..y).for_each(|y_check| {
                if map[y_check].iter().all(|c| *c == '.') {
                    y_offset += mult;
                }
            });

            if map[y][x] == '#' {
                locations.push((y + y_offset, x + x_offset));
            }
        }
        if !found {
            x_offset += mult;
        }
    }

    locations

}

fn process_for_num(input: &str, num: usize) -> String {
    let map = to_char_map(input);
    let locations = char_map_to_expanded_pos(&map, num);
    let mut set = HashSet::with_capacity(locations.len().pow(2));
    let mut sum = 0;
    for a in 0..locations.len() {
        for b in 0..locations.len() {
            if a == b {
                continue;
            }
            let calc = a + b;
            let smaller = usize::min(a, b);
            let bigger = calc - smaller;
            if !set.contains(&(smaller, bigger)) {
                set.insert((smaller, bigger));
                let a = locations[a];
                let b = locations[b];
                let diff = (a.0.abs_diff(b.0), a.1.abs_diff(b.1));
                sum += diff.0 + diff.1;
            }
        }
    }
    sum.to_string()
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    process_for_num(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("1030", process_for_num(input, 10));
        assert_eq!("8410", process_for_num(input, 100));
    }
}
