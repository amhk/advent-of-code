#!/bin/bash
set -e

script="$(readlink -f "$0")"
script_dir="$(dirname "$script")"
cd "${script_dir}/.." >/dev/null

if [[ "$1" ]]; then
    name="$1"
else
    name="aoc-$(date +"%Y-%m-%d")"
fi

if ! [[ "$name" =~ ^aoc-[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
    echo "error: bad pattern"
    exit 1
fi

tmpfile=$(mktemp)
if [[ "${AOC_SESSION}" ]]; then
    year="${name:4:4}"
    day="${name:12:2}"
    url="https://adventofcode.com/${year}/day/${day}/input"
    curl \
        -X GET \
        -H "Cookie: session=${AOC_SESSION}" \
        -o "${tmpfile}" \
        "${url}"
fi

# Point of no return: commence write operations

# - Input and test input
mkdir -p "${name}/src"
mv "$tmpfile" "${name}"/src/input.txt
touch "${name}"/src/test-input.txt

# - <day>/Cargo.toml
cat >"${name}/Cargo.toml" <<EOF
[package]
name = "${name}"
version = "0.1.0"
authors = ["Mårten Kongstad <marten.kongstad@gmail.com>"]
edition = "2018"

[dependencies]
EOF

# - <day>/src/main.rs
cat >"${name}/src/main.rs" <<EOF
#![allow(dead_code, unused_variables)]

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

fn part_one(input: &str) -> Result<usize, Error> {
    todo!();
}

fn part_two(input: &str) -> Result<usize, Error> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(0));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(0));
    }
}
EOF
