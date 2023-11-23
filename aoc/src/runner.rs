use anyhow::{bail, Result};
use atty::Stream;
use std::io::Write;
use std::ops::RangeBounds;
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(1);

const ANSI_GREEN: &str = "\x1B[32m";
const ANSI_RED: &str = "\x1B[1;31m";
const ANSI_RESET: &str = "\x1B[0m";
const ANSI_CYAN: &str = "\x1B[36m";
const ANSI_YELLOW: &str = "\x1B[33m";

pub fn run<T>(called_from: &str, func: impl FnOnce() -> Result<T>) -> Result<()>
where
    T: std::fmt::Display,
    T: PartialOrd,
{
    print_label(called_from);
    match func() {
        Ok(value) => {
            print_maybe(&value.to_string());
            Ok(())
        }
        Err(e) => {
            print_error(&e.to_string());
            Err(e)
        }
    }
}

pub fn run_with_expected_value<T>(
    called_from: &str,
    func: impl FnOnce() -> Result<T>,
    expected_value: T,
) -> Result<()>
where
    T: std::fmt::Display,
    T: PartialEq,
{
    print_label(called_from);
    let value = match func() {
        Ok(value) => value,
        Err(e) => {
            print_error(&e.to_string());
            return Err(e);
        }
    };
    if expected_value != value {
        let msg = format!("{} \u{22} {}", value, expected_value);
        print_error(&msg);
        bail!(format!("answer does not match expected value: {}", msg));
    }
    print_ok(&value.to_string());
    Ok(())
}

pub fn run_with_expected_range<T, R>(
    called_from: &str,
    func: impl FnOnce() -> Result<T>,
    expected_range: R,
) -> Result<()>
where
    T: std::fmt::Display,
    T: PartialOrd,
    R: RangeBounds<T>,
    R: std::fmt::Debug,
{
    print_label(called_from);
    let value = match func() {
        Ok(value) => value,
        Err(e) => {
            print_error(&e.to_string());
            return Err(e);
        }
    };
    if !expected_range.contains(&value) {
        let msg = format!("{} \u{2209} {:?}", value, expected_range);
        print_error(&msg);
        bail!(format!("answer not in expected range: {}", msg));
    }
    print_ok(&value.to_string());
    Ok(())
}

pub fn run_with_expected_custom_check<T>(
    called_from: &str,
    func: impl FnOnce() -> Result<T>,
    check: impl FnOnce(&T) -> bool,
) -> Result<()>
where
    T: std::fmt::Display,
    T: PartialEq,
{
    print_label(called_from);
    let value = match func() {
        Ok(value) => value,
        Err(e) => {
            print_error(&e.to_string());
            return Err(e);
        }
    };
    if !check(&value) {
        print_error("custom check failed");
        bail!("custom check failed for answer {}", value);
    }
    print_ok(&value.to_string());
    Ok(())
}

fn print_label(called_from: &str) {
    let part_no = GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
    let label = if called_from.len() >= 7 {
        format!(
            "aoc-{}-{} part {}",
            &called_from[0..4],
            &called_from[5..7],
            part_no
        )
    } else {
        format!("part {}", part_no)
    };
    print!("{}: ", label);
    std::io::stdout().flush().unwrap();
}

fn print_error(msg: &str) {
    if is_tty() {
        println!(
            "{}{:20}{} [{}FAIL{}]",
            ANSI_CYAN, msg, ANSI_RESET, ANSI_RED, ANSI_RESET,
        );
    } else {
        println!("{:20} [FAIL]", msg);
    }
}

fn print_ok(msg: &str) {
    if is_tty() {
        println!(
            "{}{:20}{} [ {}OK{} ]",
            ANSI_CYAN, msg, ANSI_RESET, ANSI_GREEN, ANSI_RESET,
        );
    } else {
        println!("{:20} [ OK ]", msg);
    }
}

fn print_maybe(msg: &str) {
    if is_tty() {
        println!(
            "{}{:20}{} [ {}??{} ]",
            ANSI_CYAN, msg, ANSI_RESET, ANSI_YELLOW, ANSI_RESET,
        );
    } else {
        println!("{:20} [ ?? ]", msg);
    }
}

fn is_tty() -> bool {
    atty::is(Stream::Stdout)
}
