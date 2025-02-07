// conjecture.rs
pub fn goldbach_conjecture() -> String {
    let mut composites = vec![];
    let mut primes = vec![true; 10000];
    primes[0] = false;
    primes[1] = false;

    for i in 2..10000 {
        if primes[i] {
            let mut multiple = i * 2;
            while multiple < 10000 {
                primes[multiple] = false;
                multiple += i;
            }
        }
    }

    for i in (9..10000).step_by(2) {
        if !primes[i] {
            composites.push(i);
        }
    }

    let mut invalid_numbers = vec![];

    for &comp in &composites {
        let mut found = false;
        for p in 2..comp {
            if primes[p] {
                let mut n = 1;
                while p + 2 * n * n <= comp {
                    if p + 2 * n * n == comp {
                        found = true;
                        break;
                    }
                    n += 1;
                }
            }
            if found {
                break;
            }
        }

        if !found {
            invalid_numbers.push(comp);
            if invalid_numbers.len() == 2 {
                break;
            }
        }
    }

    format!("{},{}", invalid_numbers[0], invalid_numbers[1])
}
