pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let (num_part, from_base_part) = match num_str.find('(') {
        Some(index) => {
            let num = &num_str[0..index];
            let base_str = &num_str[index + 1..num_str.len() - 1];
            let base = base_str.parse::<u32>().unwrap();
            (num, base)
        }
        None => (num_str, 10),
    };

    let decimal_num = u32::from_str_radix(num_part, from_base_part).unwrap();

    let mut result = String::new();
    let mut quotient = decimal_num;
    if quotient == 0 {
        result.push('0');
    } else {
        while quotient > 0 {
            let remainder = quotient % to_base;
            if remainder < 10 {
                result.push((remainder as u8 + b'0') as char);
            } else {
                result.push((remainder as u8 - 10 + b'a') as char);
            }
            quotient /= to_base;
        }
        result = result.chars().rev().collect();
    }

    result
}
