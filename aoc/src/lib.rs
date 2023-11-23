mod graph;
mod runner;

pub use graph::Graph;
pub use runner::run;
pub use runner::run_with_expected_custom_check;
pub use runner::run_with_expected_range;
pub use runner::run_with_expected_value;

#[macro_export]
macro_rules! run {
    ($expr: expr) => {{
        aoc::run(file!(), || $expr)
    }};
    ($expr: expr, $expectation: literal) => {{
        aoc::run_with_expected_value(file!(), || $expr, $expectation)
    }};
    ($expr: expr, $expectation: expr) => {{
        aoc::run_with_expected_range(file!(), || $expr, $expectation)
    }};
}

#[macro_export]
macro_rules! run_custom_check {
    ($expr: expr, $custom_check: expr) => {{
        aoc::run_with_expected_custom_check(file!(), || $expr, $custom_check)
    }};
}
