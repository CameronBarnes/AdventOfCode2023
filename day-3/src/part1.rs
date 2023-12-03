
#[tracing::instrument]
pub fn process(input: &str) -> String {
    let input_lines : Vec<&str> = input.lines().collect();

    let mut sum = 0;

    for i in 0..input_lines.len() {
        let mut line = input_lines[i];
        let mut index = 0;
        while let Some(start) = line.find(char::is_numeric) {
            let end = start + line[start..].find(|c: char| !c.is_numeric() ).unwrap_or(line.len() - 1);
            //println!("Line number: {i}, index: {index}, start: {}, end: {}", start + index, end + index);
            let min = i32::max(0, (start + index) as i32 - 1) as usize;
            let max = i32::min((line.len() + index) as i32 - 1, (end + index) as i32) as usize;
            //println!("min: {min}, max: {max}");

            if input_lines[i32::max(0, i as i32 - 1) as usize][min..=max].find(|c: char| !c.is_numeric() && c != '.').is_some() ||
                input_lines[i][min..=max].find(|c: char| !c.is_numeric() && c != '.').is_some() ||
                input_lines[usize::min(input_lines.len() - 1, i + 1)][min..=max].find(|c: char| !c.is_numeric() && c != '.').is_some() {

                    let num = if line.len() <= end {
                        line[start..].to_string()
                    } else {
                        line[start..end].to_string()
                    };
                    //println!("{num}");
                    sum += num.parse::<i64>().expect("char::is_numeric requires this be a number");

            }
            if line.len() <= end {
                line = "";
            } else {
                line = line.split_at(end).1;
            }
            index += end;
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
        assert_eq!("4361", process(input));
    }
}
