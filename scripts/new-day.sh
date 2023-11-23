#!/bin/bash
set -e

script="$(readlink -f "$0")"
script_dir="$(dirname "$script")"
cd "${script_dir}/.." >/dev/null

if [[ "$1" && "$2" ]]; then
    year="$1"
    day="$2"
else
    year="$(date +"%Y")"
    day="$(date +"%d")"
fi

if ! [[ "$year" =~ [0-9]{4} ]]; then
    echo "error: ${year} bad <year> pattern"
    exit 1
fi

if ! [[ "$day" =~ [0-9]{2} ]]; then
    echo "error: ${day} bad <day> pattern"
    exit 1
fi

dir="${year}/${day}"

tmpfile=$(mktemp)
if [[ "${AOC_SESSION}" ]]; then
    url="https://adventofcode.com/${year}/day/${day#0}/input"
    curl \
        -X GET \
        -H "Cookie: session=${AOC_SESSION}" \
        -o "${tmpfile}" \
        "${url}"
fi

# Point of no return: commence write operations

# - Input and test input
mkdir -p "${dir}/src"
mv "$tmpfile" "${dir}"/src/input.txt
touch "${dir}"/src/test-input.txt

# - <day>/Cargo.toml
cat >"${dir}/Cargo.toml" <<EOF
[package]
name = "aoc-${year}-${day}"
version = "0.1.0"
authors = ["Mårten Kongstad <marten.kongstad@gmail.com>"]
edition = "2021"

[dependencies]
anyhow = "1.0.66"
aoc = { path = "../../aoc" }
EOF

# - <day>/src/main.rs
cat >"${dir}/src/main.rs" <<EOF
use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input))?;
    aoc::run!(part_two(input))?;
    Ok(())
}

fn part_one(_input: &str) -> Result<usize> {
    todo!();
}

fn part_two(_input: &str) -> Result<usize> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 0);
    }
}
EOF
