use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
    BadPipe,
    None,
}

impl Pipe {
    fn is_pipe(&self) -> bool {
        *self != Pipe::None && *self != Pipe::BadPipe
    }
    fn is_pipe_edge(&self) {
        *self != 
    }
    fn is_none(&self) -> bool {
        *self == Pipe::None
    }
}

fn char_to_pipe(c: char) -> Pipe {
    use Pipe::*;
    match c {
        '|' => NS,
        '-' => EW,
        'L' => NE,
        'J' => NW,
        '7' => SW,
        'F' => SE,
        'S' => Start,
        '.' => None,
        value => panic!("Invalid input: {value}")
    }
}

fn parse_line(line: &str) -> Vec<Pipe> {
    line.chars().map(char_to_pipe).collect()
}

fn get_start_pos(map: &Vec<Vec<Pipe>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Pipe::Start {
                return (y, x);
            }
        }
    }
    panic!("No Start")
}

fn bound(start: usize, max: usize, diff: i32) -> usize {
    i32::max(0, i32::min(max as i32, start as i32 + diff)) as usize
}

fn check_pos_connects_to(start: &(usize, usize), pos: &(usize, usize), map: &[Vec<Pipe>]) -> bool {

    use Pipe::*;

    //println!("start: {:?}, check: {:?}", start, pos);
    let pipe = map[pos.0][pos.1];
    let diff = (pos.0 as i32 - start.0 as i32, pos.1 as i32 - start.1 as i32);
    let result = match diff {
        (-1, 0) => { // Above
            pipe == NS || pipe == SW || pipe == SE || pipe == Start
        },
        (0, -1) => { // Left
            pipe == EW || pipe == NE || pipe == SE || pipe == Start
        },
        (0, 1) => { // Right
            pipe == EW || pipe == SW || pipe == NW || pipe == Start
        },
        (1, 0) => { // Bellow
            pipe == NS || pipe == NE || pipe == NW || pipe == Start
        },
        (0, 0) => false,
        value => panic!("Unexpected value: {:?}", value)
    };
    //println!("start: {:?},pos: {:?}, pipe: {:?}, result: {result}", start, pos, pipe);
    //println!("Start Pipe: {:?}, Start Pos: {:?}, Pos Pipe: {:?}, Pos Pos: {:?}, result: {result}", map[start.0][start.1], start, pipe, pos);
    result

}

const AROUND: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn find_valid_next_pos(start: &(usize, usize), map: &Vec<Vec<Pipe>>, is_start: bool) -> Vec<(usize, usize)> {

    let max_y = map.len();
    let max_x = map[0].len();
    let mut valid = HashSet::new();
    
    if is_start {
        for pos in AROUND {
            let pos = (bound(start.0, max_y, pos.0), bound(start.1, max_x, pos.1));
            if check_pos_connects_to(start, &pos, map) {
                valid.insert(pos);
            }
        }
    } else {
        let mut opts = Vec::new();
        match map[start.0][start.1] {
            Pipe::NS => {
                opts.push((-1, 0));
                opts.push((1, 0));
            },
            Pipe::EW => {
                opts.push((0, -1));
                opts.push((0, 1));
            },
            Pipe::NE => {
                opts.push((-1, 0));
                opts.push((0, 1));
            },
            Pipe::NW => {
                opts.push((-1, 0));
                opts.push((0, -1));
            },
            Pipe::SW => {
                opts.push((1, 0));
                opts.push((0, -1));
            },
            Pipe::SE => {
                opts.push((1, 0));
                opts.push((0, 1));
            },
            Pipe::Start | Pipe::None | Pipe::BadPipe => panic!("Invalid current pos"),
        }
        for pos in opts {
            let pos2 = (bound(start.0, max_y, pos.0), bound(start.1, max_x, pos.1));
            //println!("start: {:?},pos1: {:?}, pos2: {:?}", start, pos, pos2);
            let is_valid = check_pos_connects_to(start, &pos2, map);
            //println!("start: {:?}, pos2: {:?}, is_valid: {is_valid}", start, pos2);
            if is_valid {
                valid.insert(pos2);
            }
            //println!();
        }
    }

    valid.into_iter().collect()
}

fn find_enclosed_spaces(mut map: Vec<Vec<Pipe>>) -> usize {

    let start = get_start_pos(&map);
    let start_options = find_valid_next_pos(&start, &map, true);
    let first_try_route = start_options[0];
    let mut in_loop = vec!(vec!(false; map[0].len()); map.len());
    in_loop[start.0][start.1] = true;

    let mut last_pos = (usize::MAX, usize::MAX);
    let mut cur_pos = first_try_route;
    in_loop[cur_pos.0][cur_pos.1] = true;
    loop {
        let options = find_valid_next_pos(&cur_pos, &map, false);
        //println!("Cur_pos: {:?}, options: {:?}\n", cur_pos, options);
        if cur_pos != first_try_route && options.contains(&start) {
            //println!("Start break");
            break;
        }
        if options[0] != last_pos && options[0] != start {
            last_pos = cur_pos;
            cur_pos = options[0];
        } else if options[1] != start {
            last_pos = cur_pos;
            cur_pos = options[1];
        } else {
            panic!("Start where it's not supposed to be");
        }
        in_loop[cur_pos.0][cur_pos.1] = true;
        //println!("Here!");
        
        /*for line in &dists {
            println!("{:?}", line);
        }*/ 
    }
    
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !in_loop[y][x] {
                map[y][x] = Pipe::BadPipe;
            }
        }
    }

    for y in 0..map.len() {
        let mut in_loop = false;
        let mut last_was_pipe = false;
        for x in 0..map[0].len() {
            let marker = map[y][x];
            if !last_was_pipe
        }
    }

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let map: Vec<Vec<Pipe>> = input.lines().map(parse_line).collect();
    find_enclosed_spaces(map).to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........", "4")]
    #[case("..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........", "4")]
    #[case(".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...", "8")]
    #[case("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L", "10")]
    fn test_process(#[case] input: &str, #[case] result: &str) {
        assert_eq!(result.to_string(), process(input));
    }
}
