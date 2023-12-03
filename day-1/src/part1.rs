
fn parse_line(str: &str) -> i32 {

    let matches : Vec<&str> = str.matches(char::is_numeric).collect();
    let mut out = matches[0].to_string();
    out.push_str(matches[matches.len() - 1]);

    out.parse().unwrap()

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    input.lines().map(parse_line).sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input));
    }
}
