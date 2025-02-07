pub fn new_birthday_probability(n: u32) -> f64 {
    let mut probability_all_different = 1.0;
    for i in 0..n {
        probability_all_different *= (365.0 - i as f64) / 365.0;
    }
    1.0 - probability_all_different
}
