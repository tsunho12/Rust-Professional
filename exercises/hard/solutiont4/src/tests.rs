mod calc_time;

#[cfg(test)]
mod tests {
    use super::calc_time::time_info;
    use std::time::{Duration, Instant};

    const TEST_CASES: &[(&str, &str)] = &[
        ("2025-01-01", "1,3,1,364,27,0"),
        ("2025-01-18", "2,6,18,347,10,1"),
    ];

    #[test]
    fn test_calc_time() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let start = Instant::now();
            let result = time_info(*input);
            let duration = start.elapsed();

            // 调试输出
            println!("Input: {}, Expected: {}, Got: {}", input, expected, result);

            if duration <= Duration::from_millis(200) && result == *expected {
                total_score += 10.0;
            }
        }

        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
