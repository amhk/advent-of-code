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

if [[ $(grep -c -e "$name" Cargo.toml) -gt 0 || -e "${name}" ]]; then
    echo "error: duplicate pattern"
    exit 1
fi

# Point of no return: commence write operations
# - Cargo.toml
sed -i "s/^    # <template>/    \"${name}\",\n    # <template>/" Cargo.toml

# - Input and test input
mkdir -p "${name}/src"
touch "${name}"/src/{,test-}input.txt

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

    let answer = part_one(&input);
    println!("part 1: {}", answer);

    let answer = part_two(&input);
    println!("part 2: {}", answer);
}

fn part_one(input: &str) -> i64 {
    todo!();
}

fn part_two(input: &str) -> i64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 0);
    }
}
EOF
