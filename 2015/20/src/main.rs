use primes::{factors_uniq, is_prime};
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let input = 34_000_000;

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

fn presents_for_house_part_one(house_number: u64) -> u64 {
    debug_assert_ne!(house_number, 0);

    // case one: house number is 1
    if house_number == 1 {
        return 10;
    }

    // case two: house number is a prime number
    if is_prime(house_number) {
        return 10 * (1 + house_number);
    }

    // case three: house number is not a prime number
    let mut v = vec![1, house_number];
    for factor in factors_uniq(house_number) {
        for i in 1.. {
            let number = factor * i;
            if number >= house_number {
                break;
            }
            if house_number % number == 0 && !v.contains(&number) {
                v.push(number);
            }
        }
    }
    10 * v.iter().sum::<u64>()
}

fn presents_for_house_part_two(house_number: u64) -> u64 {
    debug_assert_ne!(house_number, 0);

    // case one: house number is 1
    if house_number == 1 {
        return 11;
    }

    // case two: house number is a prime number
    if is_prime(house_number) {
        if house_number <= 50 {
            return 11 * (1 + house_number);
        } else {
            return 11 * house_number;
        }
    }

    // case three: house number is not a prime number
    let mut v = vec![house_number];
    if house_number <= 50 {
        v.push(1);
    }
    for factor in factors_uniq(house_number) {
        for i in 1.. {
            let number = factor * i;
            if number >= house_number {
                break;
            }
            if house_number / number <= 50 && house_number % number == 0 && !v.contains(&number) {
                v.push(number);
            }
        }
    }
    11 * v.iter().sum::<u64>()
}

struct Context {
    result: Option<u64>,
    next: u64,
    step_size: u64,
}

fn worker_thread<F>(ctx: Arc<Mutex<Context>>, target_number_of_presents: u64, func: F)
where
    F: Fn(u64) -> u64 + Copy + Send + 'static,
{
    loop {
        let range = {
            let mut ctx = ctx.lock().unwrap();
            if ctx.result.is_some() {
                return;
            }
            let range = ctx.next..=(ctx.next + ctx.step_size);
            ctx.next += ctx.step_size;
            range
        };
        for i in range {
            if func(i) >= target_number_of_presents {
                let mut ctx = ctx.lock().unwrap();
                if let Some(j) = ctx.result {
                    if i < j {
                        ctx.result = Some(i);
                        return;
                    }
                } else {
                    ctx.result = Some(i);
                    return;
                }
            }
        }
    }
}

fn find_first_house<F>(target_number_of_presents: u64, func: F) -> Option<u64>
where
    F: Fn(u64) -> u64 + Copy + Send + 'static,
{
    let shared_ctx = Arc::new(Mutex::new(Context {
        result: None,
        next: 1,
        step_size: 1000,
    }));

    let mut threads = Vec::new();
    for _ in 0..num_cpus::get() {
        let ctx = shared_ctx.clone();
        threads.push(thread::spawn(move || {
            worker_thread(ctx, target_number_of_presents, func);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    let result = shared_ctx.lock().unwrap().result;
    result
}

fn part_one(target_number_of_presents: u64) -> Option<u64> {
    find_first_house(target_number_of_presents, presents_for_house_part_one)
}

fn part_two(target_number_of_presents: u64) -> Option<u64> {
    find_first_house(target_number_of_presents, presents_for_house_part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presents_for_house_part_one() {
        assert_eq!(presents_for_house_part_one(1), 10);
        assert_eq!(presents_for_house_part_one(2), 30);
        assert_eq!(presents_for_house_part_one(3), 40);
        assert_eq!(presents_for_house_part_one(4), 70);
        assert_eq!(presents_for_house_part_one(5), 60);
        assert_eq!(presents_for_house_part_one(6), 120);
        assert_eq!(presents_for_house_part_one(7), 80);
        assert_eq!(presents_for_house_part_one(8), 150);
        assert_eq!(presents_for_house_part_one(9), 130);
        assert_eq!(presents_for_house_part_one(20), 420);
        assert_eq!(presents_for_house_part_one(21), (1 + 3 + 7 + 21) * 10);
        assert_eq!(presents_for_house_part_one(700000), 19686240);
        assert_eq!(presents_for_house_part_one(780000), 27553680);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(420), Some(20));
    }
}
