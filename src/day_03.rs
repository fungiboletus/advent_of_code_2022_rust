pub fn parse_input_data(input: &str) -> Vec<Vec<u8>> {
    return input
        .lines()
        .map(|backpack_line| {
            // Split line by characters
            return backpack_line
                .chars()
                .into_iter()
                .map(|character| {
                    // Convert each character to the "priority"
                    // a-z => 1-26
                    // A-Z => 27-52
                    return match character {
                        'a'..='z' => character as u8 - 'a' as u8 + 1,
                        'A'..='Z' => character as u8 - 'A' as u8 + 27,
                        _ => panic!("Invalid character"),
                    };
                })
                .collect();
        })
        .collect();
}

pub fn find_duplicate_number(list: Vec<u8>) -> Result<u8, String> {
    // We know the numbers are betwen 1 and 52
    // We use a set to keep track of which numbers we've seen
    // But since it's so few numbers, a boolean array is faster
    let mut hit_table = vec![false; 52];

    // Fill the table with the first half of the list
    for number in list.iter().take(list.len() / 2) {
        hit_table[*number as usize - 1] = true;
    }

    // Check which number is in the table in the second half of the list
    for number in list.iter().skip(list.len() / 2) {
        if hit_table[*number as usize - 1] {
            return Ok(*number);
        }
    }

    return Result::Err("No duplicate found".to_string());
}

pub fn day_3_part_1(data: &str) -> i64 {
    let backpacks = parse_input_data(data);

    let sum: i64 = backpacks
        .iter()
        .map(|backpack| {
            return find_duplicate_number(backpack.clone()).unwrap() as i64;
        })
        .sum();

    return sum;
}

pub fn day_3_part_2(data: &str) -> i64 {
    let backpacks = parse_input_data(data);

    return backpacks.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_day_3_part_1() {
        assert_eq!(day_3_part_1(EXAMPLE), 157);
    }

    #[test]
    fn test_day_3_part_2() {
        assert_eq!(day_3_part_2(EXAMPLE), 12);
    }
}
