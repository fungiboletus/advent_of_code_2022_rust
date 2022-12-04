#[derive(Debug)]
pub struct Range {
    start: i64,
    end: i64,
}

pub fn parse_input_data(input: &str) -> Vec<(Range, Range)> {
    return input
        .lines()
        .map(|line| {
            // format
            // 2-4,6-8
            // Range<start, end>,Range<start, end>
            // use scanf to parse
            let parsed =
                sscanf::sscanf!(line, "{i64}-{i64},{i64}-{i64}").expect("Unable to parse line");
            let range1 = Range {
                start: parsed.0,
                end: parsed.1,
            };
            let range2 = Range {
                start: parsed.2,
                end: parsed.3,
            };
            return (range1, range2);
        })
        .collect();
}

pub fn day_4_part_1(data: &str) -> i64 {
    let ranges = parse_input_data(data);

    let mut sum = 0;
    for (range1, range2) in ranges.iter() {
        // Check if one range fits completely into the other
        if (range1.start >= range2.start && range1.end <= range2.end)
            || (range2.start >= range1.start && range2.end <= range1.end)
        {
            sum += 1;
        }
    }

    return sum;
}

pub fn day_4_part_2(data: &str) -> i64 {
    let ranges = parse_input_data(data);

    let mut sum = 0;
    for (range1, range2) in ranges.iter() {
        // Check if one range overlaps with the other
        if (range1.start <= range2.start && range1.end >= range2.start)
            || (range2.start <= range1.start && range2.end >= range1.start)
        {
            sum += 1;
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_day_4_part_1() {
        assert_eq!(day_4_part_1(EXAMPLE), 2);
    }

    #[test]
    fn test_day_4_part_2() {
        assert_eq!(day_4_part_2(EXAMPLE), 4);
    }
}
