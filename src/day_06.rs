use core::panic;

pub fn day_6_part_1(data: &str) -> usize {
    let characters: Vec<char> = data.chars().collect();
    // iterate over characters with a i variable
    for i in 3..characters.len() {
        let char_n = characters.get(i).expect("Unable to get character");
        let char_n_minus_1 = characters.get(i - 1).expect("Unable to get character - 1");
        let char_n_minus_2 = characters.get(i - 2).expect("Unable to get character - 2");
        let char_n_minus_3 = characters.get(i - 3).expect("Unable to get character - 3");
        // println!(
        //     "sequence: {}{}{}{}",
        //     char_n_minus_3, char_n_minus_2, char_n_minus_1, char_n,
        // );
        if char_n != char_n_minus_1
            && char_n != char_n_minus_2
            && char_n != char_n_minus_3
            && char_n_minus_1 != char_n_minus_2
            && char_n_minus_1 != char_n_minus_3
            && char_n_minus_2 != char_n_minus_3
        {
            return i + 1;
        }
    }
    panic!("Unable to find a solution");
}

pub fn day_6_part_2(data: &str) -> usize {
    return 42;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE_B: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE_C: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE_D: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE_E: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_day_6_part_1() {
        assert_eq!(day_6_part_1(EXAMPLE_A), 7);
        assert_eq!(day_6_part_1(EXAMPLE_B), 5);
        assert_eq!(day_6_part_1(EXAMPLE_C), 6);
        assert_eq!(day_6_part_1(EXAMPLE_D), 10);
        assert_eq!(day_6_part_1(EXAMPLE_E), 11);
    }

    #[test]
    fn test_day_6_part_2() {
        assert_eq!(day_6_part_2(EXAMPLE_A), 42);
    }
}
