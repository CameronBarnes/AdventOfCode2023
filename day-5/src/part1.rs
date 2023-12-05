#[derive(Debug)]
struct ValMap {
    src: usize,
    dest: usize,
    len: usize,
}

impl ValMap {
    fn valid_src(&self, input: usize) -> bool {
        input >= self.src && input < self.src + self.len
    }
    fn map_val(&self, input: usize) -> usize {
        if !self.valid_src(input) {
            panic!("invalid src");
        }
        let input = input - self.src;
        self.dest + input
    }

}

fn parse_line(str: &str) -> ValMap {
    if str.is_empty() {
        panic!("input is empty");
    } else if str.contains(':') {
        panic!("str contains invalid chars");
    }

    let mut iter = str.split_whitespace();
    let dest = iter.next().unwrap().parse().unwrap();
    let src = iter.next().unwrap().parse().unwrap();
    let len = iter.last().unwrap().parse().unwrap();

    ValMap{src, dest, len}

}

fn map(maps: &[ValMap], input: usize) -> usize {

    for map in maps {
        if map.valid_src(input) {
            return map.map_val(input);
        }
    }

    input

}

#[tracing::instrument]
pub fn process(input: &str) -> String {

    let mut iter = input.split("\n\n");
    let seeds_iter = iter.next().unwrap().trim().split(':').last().unwrap()
        .split_whitespace().flat_map(str::parse::<usize>);

    let mut lowest = usize::MAX;
    let mut maps: Vec<Vec<ValMap>> = Vec::with_capacity(7);
    for str in iter {
        let mut lines = str.lines();
        lines.next();
        let map: Vec<ValMap> = lines.map(parse_line).collect();
        //map.sort_by_key(|m| m.src);
        maps.push(map);
    }

    for seed in seeds_iter {
        let seed = map(&maps[0], seed);
        let seed = map(&maps[1], seed);
        let seed = map(&maps[2], seed);
        let seed = map(&maps[3], seed);
        let seed = map(&maps[4], seed);
        let seed = map(&maps[5], seed);
        let seed = map(&maps[6], seed);
        lowest = usize::min(lowest, seed);
    }

    lowest.to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("35", process(input));
    }
}
