mod bounding_box;
mod graph;
mod parse;
mod runner;
mod xy;

pub use bounding_box::BoundingBox;
pub use graph::Graph;
pub use parse::parse1;
pub use parse::parse2;
pub use parse::parse3;
pub use parse::parse4;
pub use parse::parse5;
pub use parse::parse6;
pub use parse::parse7;
pub use parse::parse8;
pub use parse::parse_grid;
pub use runner::run;
pub use runner::run_with_expected_custom_check;
pub use runner::run_with_expected_range;
pub use runner::run_with_expected_value;
pub use xy::XY;

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

#[macro_export]
macro_rules! parse {
    ($regex:expr, $haystack:expr, $to_a:expr) => {
        aoc::parse1($regex, $haystack, $to_a)
    };
    ($regex:expr, $haystack:expr, $to_a:expr, $to_b:expr) => {
        aoc::parse2($regex, $haystack, $to_a, $to_b)
    };
    ($regex:expr, $haystack:expr, $to_a:expr, $to_b:expr, $to_c:expr) => {
        aoc::parse3($regex, $haystack, $to_a, $to_b, $to_c)
    };
    ($regex:expr, $haystack:expr, $to_a:expr, $to_b:expr, $to_c:expr, $to_d:expr) => {
        aoc::parse4($regex, $haystack, $to_a, $to_b, $to_c, $to_d)
    };
    ($regex:expr, $haystack:expr, $to_a:expr, $to_b:expr, $to_c:expr, $to_d:expr, $to_e:expr) => {
        aoc::parse5($regex, $haystack, $to_a, $to_b, $to_c, $to_d, $to_e)
    };
    ($regex:expr, $haystack:expr, $to_a:expr, $to_b:expr, $to_c:expr, $to_d:expr, $to_e:expr, $to_f:expr) => {
        aoc::parse6($regex, $haystack, $to_a, $to_b, $to_c, $to_d, $to_e, $to_f)
    };
    ($regex:expr, $haystack:expr, $to_a:expr, $to_b:expr, $to_c:expr, $to_d:expr, $to_e:expr, $to_f:expr, $to_g:expr) => {
        aoc::parse7(
            $regex, $haystack, $to_a, $to_b, $to_c, $to_d, $to_e, $to_f, $to_g,
        )
    };
    ($regex:expr, $haystack:expr, $to_a:expr, $to_b:expr, $to_c:expr, $to_d:expr, $to_e:expr, $to_f:expr, $to_g:expr, $to_h:expr) => {
        aoc::parse8(
            $regex, $haystack, $to_a, $to_b, $to_c, $to_d, $to_e, $to_f, $to_g, $to_h,
        )
    };
}
