use std::collections::HashSet;

use itertools::Itertools;


struct Map {
    map: Vec<Vec<u16>>
}

impl Map {
    fn to_string(&self) -> String {
        let mut str = String::new();
        self.map.iter().for_each(|line| {
            str.push_str(&line.iter().map(|num| {
                if *num > 0 {
                    '#'
                } else {
                    '.'
                }
            }).join(""));
            str.push('\n');
        });
        str.trim().to_string()
    }

    fn new(input: &str) -> Map {
        let mut counter: u16 = 0;
        let map = input.lines().map(|line| {
            line.chars().map(|c| {
                if c == '#' {
                    counter += 1;
                    counter
                } else {
                    0
                }
            }).collect()
        }).collect();
        Map{map}
    }
    fn empty_rows(&self) -> Vec<usize> {
        let mut rows = Vec::new();
        for y in (0..self.map.len()).rev() {
            if self.map[y].iter().all(|num| *num == 0) {
                rows.push(y);
            }
        }
        rows
    }
    fn empty_cols(&self) -> Vec<usize> {
        let mut cols = Vec::new();
        let max_y = self.map.len();
        for x in (0..self.map[0].len()).rev() {
            let mut good = true;
            for y in 0..max_y {
                if self.map[y][x] > 0 {
                    good = false;
                    break;
                }
            }
            if good {
                cols.push(x);
            }
        }
        cols
    }

    fn get_locations(&self) -> Vec<(usize, usize)> {
        let mut found = Vec::new();
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if self.map[y][x] > 0 {
                    found.push((y, x));
                }
            }
        }
        found
    }
}

fn expand_map(mut map: Map) -> Map {

    // Lets expand rows first
    let start_max_x = map.map[0].len();
    for row in map.empty_rows() {
        map.map.insert(row, vec!(0; start_max_x));
    }

    // Next we'll expand the columns
    let start_max_y = map.map.len();
    for col in map.empty_cols() {
        for y in 0..start_max_y {
            map.map[y].insert(col, 0);
        }
    }

    map

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let map = Map::new(input);
    let map = expand_map(map);
    let locations = map.get_locations();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expansion() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(
"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......", expand_map(Map::new(input)).to_string());
    }

    #[test]
    fn test_process() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("374", process(input));
    }
}
