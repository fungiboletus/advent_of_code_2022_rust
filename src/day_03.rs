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

pub fn find_number_in_both_halfes_of_the_list(list: Vec<u8>) -> Result<u8, String> {
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

    return Result::Err("No number found".to_string());
}

pub fn find_common_number_in_three_lists(
    list1: Vec<u8>,
    list2: Vec<u8>,
    list3: Vec<u8>,
) -> Result<u8, String> {
    let mut hit_table: Vec<u8> = vec![0; 52];

    // Fill the table with the first list
    for number in list1.iter() {
        hit_table[*number as usize - 1] = 1;
    }

    // Fill the table with the second list
    for number in list2.iter() {
        let index = *number as usize - 1;
        if hit_table[index] == 1 {
            hit_table[index] = 2;
        }
    }

    // Check which number is in the table from the third list
    for number in list3.iter() {
        let index = *number as usize - 1;
        if hit_table[index] == 2 {
            return Ok(*number);
        }
    }

    return Result::Err("No number found".to_string());
}

pub fn day_3_part_1(data: &str) -> i64 {
    let backpacks = parse_input_data(data);

    let sum: i64 = backpacks
        .iter()
        .map(|backpack| {
            return find_number_in_both_halfes_of_the_list(backpack.clone()).unwrap() as i64;
        })
        .sum();

    return sum;
}

pub fn day_3_part_2(data: &str) -> i64 {
    let backpacks = parse_input_data(data);

    // Iterate the vector in chunks of 3
    let sum: i64 = backpacks
        .chunks(3)
        .map(|backpacks| {
            return find_common_number_in_three_lists(
                backpacks[0].clone(),
                backpacks[1].clone(),
                backpacks[2].clone(),
            )
            .unwrap() as i64;
        })
        .sum();

    return sum;
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
        assert_eq!(day_3_part_2(EXAMPLE), 70);
    }
}
