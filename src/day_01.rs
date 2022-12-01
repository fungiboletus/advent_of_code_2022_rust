pub fn parse_input_data(input: &str) -> Vec<Vec<i64>> {
    return input
        // Split the string every two end of line characters
        .split("\n\n")
        .map(|elf| {
            return elf
                // Split the string into lines
                .lines()
                // Convert each line to a vector of i64
                .map(|line| line.parse::<i64>().expect("Failed to parse number"))
                .collect();
        })
        .collect();
}

pub fn day_1_part_1(data: &str) -> i64 {
    let data = parse_input_data(data);

    let sums = data.iter().map(|elf| {
        return elf.iter().sum();
    });

    let biggest_sum = sums.max().expect("The dataset doesn't contain any numbers");

    return biggest_sum;
}

pub fn day_1_part_2(data: &str) -> i64 {
    let data = parse_input_data(data);

    let mut max_a = 0;
    let mut max_b = 0;
    let mut max_c = 0;

    for elf in data {
        let calories = elf.iter().sum();
        if calories > max_a {
            max_c = max_b;
            max_b = max_a;
            max_a = calories;
        } else if calories > max_b {
            max_c = max_b;
            max_b = calories;
        } else if calories > max_c {
            max_c = calories;
        }
    }

    return max_a + max_b + max_c;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_day_1_part_1() {
        assert_eq!(day_1_part_1(EXAMPLE), 24000);
    }

    #[test]
    fn test_day_1_part_2() {
        assert_eq!(day_1_part_2(EXAMPLE), 45000);
    }
}
