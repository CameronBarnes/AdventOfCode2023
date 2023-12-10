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
    None,
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

    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;
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
            Pipe::Start | Pipe::None => panic!("Invalid current pos"),
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

fn find_farthest_pipe(map: &Vec<Vec<Pipe>>) -> usize {

    let start = get_start_pos(map);
    let mut dists: Vec<Vec<i32>> = vec!(vec!(-1; map[0].len()); map.len());
    dists[start.0][start.1] = 0;
    let start_options = find_valid_next_pos(&start, map, true);
    let first_try_route = start_options[0];
    dists[first_try_route.0][first_try_route.1] = 1;

    let mut step = 1;
    let mut last_pos = (usize::MAX, usize::MAX);
    let mut cur_pos = first_try_route;
    loop {
        let options = find_valid_next_pos(&cur_pos, map, false);
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
        //println!("Here!");
        step += 1;
        let marker = &mut dists[cur_pos.0][cur_pos.1];
        //println!("Marker: {marker}, step: {step}");
        if *marker != -1 && *marker < step {
            //println!("Marker Break");
            break;
        } else {
            *marker = step;
        }
        /*for line in &dists {
            println!("{:?}", line);
        }*/
    }
    step = 1;
    last_pos = (usize::MAX, usize::MAX);
    cur_pos = start_options[1];
    let second_try_route = cur_pos;
    dists[second_try_route.0][second_try_route.1] = 1;
    loop {
        let options = find_valid_next_pos(&cur_pos, map, false);
        //println!("Cur_pos: {:?}, options: {:?}\n", cur_pos, options);
        if cur_pos != second_try_route && options.contains(&start) {
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
        step += 1;
        let marker = &mut dists[cur_pos.0][cur_pos.1];
        //println!("Marker: {marker}, step: {step}");
        if *marker != -1 && *marker < step {
            //println!("Marker Break");
            break;
        } else {
            *marker = step;
        }
        /* for line in &dists {
            println!("{:?}", line);
        } */
    }
   
    /*for line in &dists {
        println!("{:?}", line);
    }*/
    *dists.iter().flat_map(|row| row.iter()).max().unwrap() as usize

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let map: Vec<Vec<Pipe>> = input.lines().map(parse_line).collect();
    find_farthest_pipe(&map).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_process() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!("4", process(input));
    }

    #[test]
    fn test2_process() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!("8", process(input));
    }

    #[test]
    fn test3_process() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!("4", process(input));
    }
}
