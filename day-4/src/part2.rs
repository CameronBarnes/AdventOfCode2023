
fn parse_card(line: &str) -> usize {

    let iter = line.split(':');
    let numbers_line = iter.last().expect("All lines should be seperated by 1 colon");

    let mut numbers_line_iter = numbers_line.split('|');
    let your_numbers: Vec<i32> = numbers_line_iter.next().unwrap()
        .split_whitespace().flat_map(str::parse::<i32>).collect();
    let winning_numbers: Vec<i32> = numbers_line_iter.next().unwrap()
        .split_whitespace().flat_map(str::parse::<i32>).collect();

    let mut score = 0;
    for num in your_numbers {
        if winning_numbers.contains(&num) {
            score += 1;
        }
    }

    score

}

#[tracing::instrument]
pub fn process(input: &str) -> String {

    let cards: Vec<usize> = input.lines().map(parse_card).collect();
    let mut num_cards = vec!(1; cards.len());

    for i in 0..cards.len() {
        let score = cards[i];

        let multiplier = num_cards[i];

        if score == 0 {
            continue;
        } else {
            (1..=score).for_each(|x| {
                num_cards[i + x] += multiplier;
            });
        }

    }

    num_cards.iter().sum::<usize>().to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input));
    }
}
