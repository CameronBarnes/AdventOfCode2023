use std::collections::HashMap;


#[tracing::instrument]
pub fn process(input: &str) -> String {
    let input_lines : Vec<&str> = input.lines().collect();

    let mut map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for i in 0..input_lines.len() {
        let mut line = input_lines[i];
        let mut index = 0;
        while let Some(start) = line.find(char::is_numeric) {
            let end = start + line[start..].find(|c: char| !c.is_numeric() ).unwrap_or(line.len() - 1);
            //println!("Line number: {i}, index: {index}, start: {}, end: {}", start + index, end + index);
            let min = i32::max(0, (start + index) as i32 - 1) as usize;
            let max = i32::min((line.len() + index) as i32 - 1, (end + index) as i32) as usize;
            //println!("min: {min}, max: {max}");
            
            let bellow = i32::max(0, i as i32 - 1) as usize;
            if let Some(gear) = input_lines[bellow][min..=max].find(|c: char| !c.is_numeric() && c != '.') {

                let num = if line.len() <= end {
                    line[start..].to_string()
                } else {
                    line[start..end].to_string()
                };

                let key = (bellow, gear + min);
                if let std::collections::hash_map::Entry::Vacant(e) = map.entry(key) {
                    e.insert(vec!(num.parse::<usize>().unwrap()));
                } else {
                    map.get_mut(&key).unwrap().push(num.parse().unwrap());
                }

            }
            if let Some(gear) = input_lines[i][min..=max].find(|c: char| !c.is_numeric() && c != '.') {
                let num = if line.len() <= end {
                    line[start..].to_string()
                } else {
                    line[start..end].to_string()
                };

                let key = (i, gear + min);
                if let std::collections::hash_map::Entry::Vacant(e) = map.entry(key) {
                    e.insert(vec!(num.parse::<usize>().unwrap()));
                } else {
                    map.get_mut(&key).unwrap().push(num.parse().unwrap());
                }
            }
            let above = usize::min(input_lines.len() - 1, i + 1);
            if let Some(gear) = input_lines[above][min..=max].find(|c: char| !c.is_numeric() && c != '.') {
                let num = if line.len() <= end {
                    line[start..].to_string()
                } else {
                    line[start..end].to_string()
                };

                let key = (above, gear + min);
                if let std::collections::hash_map::Entry::Vacant(e) = map.entry(key) {
                    e.insert(vec!(num.parse::<usize>().unwrap()));
                } else {
                    map.get_mut(&key).unwrap().push(num.parse().unwrap());
                }
            }

            if line.len() <= end {
                line = "";
            } else {
                line = line.split_at(end).1;
            }
            index += end;
        }
    }

    //println!("{:?}", map);

    let mut sum: usize = 0;
    for item in map.values() {
        if item.len() == 2 {
            sum += item[0] * item[1];
        }
    }

    sum.to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input));
    }
}
