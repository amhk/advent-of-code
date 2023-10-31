use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let input = include_str!("input.txt").trim();

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);
    assert_eq!(answer, 282_749);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
    assert_eq!(answer, 9_962_624);
}

struct Context {
    result: Option<usize>,
    next: usize,
    step_size: usize,
}

fn worker_thread<F>(secret_key: &str, verify_func: F, ctx: Arc<Mutex<Context>>)
where
    F: Fn(&md5::Digest) -> bool,
{
    loop {
        let range = {
            let mut ctx = ctx.lock().unwrap();
            if ctx.result.is_some() || ctx.next >= usize::MAX - ctx.step_size {
                return;
            }
            let range = ctx.next..=(ctx.next + ctx.step_size);
            ctx.next += ctx.step_size;
            range
        };

        for i in range {
            let mut input: Vec<u8> = vec![];
            input.extend_from_slice(secret_key.as_bytes());
            input.extend_from_slice(i.to_string().as_bytes());
            let hash = md5::compute(input);
            if verify_func(&hash) {
                let mut ctx = ctx.lock().unwrap();
                ctx.result = Some(i);
                return;
            }
        }
    }
}

fn find_number<F>(secret_key: &str, verify_func: F) -> Option<usize>
where
    F: Fn(&md5::Digest) -> bool + Copy + Send + 'static,
{
    let shared_ctx = Arc::new(Mutex::new(Context {
        result: None,
        next: 0,
        step_size: 1000,
    }));

    let mut threads = Vec::new();

    for _ in 0..num_cpus::get() {
        let ctx = shared_ctx.clone();
        let secret_key = secret_key.to_string();
        threads.push(thread::spawn(move || {
            worker_thread(&secret_key, verify_func, ctx)
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    let result = shared_ctx.lock().unwrap().result;
    result
}

fn part_one(secret_key: &str) -> Option<usize> {
    find_number(secret_key, |hash| {
        hash[0] == 0 && hash[1] == 0 && hash[2] <= 0x0f
    })
}

fn part_two(secret_key: &str) -> Option<usize> {
    find_number(secret_key, |hash| {
        hash[0] == 0 && hash[1] == 0 && hash[2] == 0
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("abcdef"), Some(609043));
        assert_eq!(part_one("pqrstuv"), Some(1048970));
    }
}
