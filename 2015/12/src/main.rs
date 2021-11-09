use serde_json::Value;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

fn parse_input(input: &str) -> Result<Value, Error> {
    serde_json::from_str(input).map_err(|_| Error::BadInput)
}

fn sum(value: &Value, exclude_pattern: Option<&str>) -> i64 {
    if let Some(number) = value.as_i64() {
        return number;
    }
    if let Some(array) = value.as_array() {
        return array
            .iter()
            .fold(0, |acc, item| acc + sum(item, exclude_pattern));
    }
    if let Some(object) = value.as_object() {
        if exclude_pattern.is_none()
            || !object
                .values()
                .map(|v| v.as_str())
                .any(|v| v == exclude_pattern)
        {
            return object
                .values()
                .fold(0, |acc, item| acc + sum(item, exclude_pattern));
        }
    }
    0
}

fn part_one(input: &str) -> Result<i64, Error> {
    Ok(sum(&parse_input(input)?, None))
}

fn part_two(input: &str) -> Result<i64, Error> {
    Ok(sum(&parse_input(input)?, Some("red")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(sum(&parse_input(r#"[1,2,3]"#).unwrap(), None), 6);
        assert_eq!(sum(&parse_input(r#"{"a":2,"b":4}"#).unwrap(), None), 6);
        assert_eq!(sum(&parse_input(r#"[[[3]]]"#).unwrap(), None), 3);
        assert_eq!(
            sum(&parse_input(r#"{"a":{"b":4},"c":-1}"#).unwrap(), None),
            3
        );
        assert_eq!(sum(&parse_input(r#"{"a":[-1,1]}"#).unwrap(), None), 0);
        assert_eq!(sum(&parse_input(r#"[-1,{"a":1}]"#).unwrap(), None), 0);
        assert_eq!(sum(&parse_input(r#"[]"#).unwrap(), None), 0);
        assert_eq!(sum(&parse_input(r#"{}"#).unwrap(), None), 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(sum(&parse_input(r#"[1,2,3]"#).unwrap(), Some("red")), 6);
        assert_eq!(
            sum(
                &parse_input(r#"[1,{"c":"red","b":2},3]"#).unwrap(),
                Some("red")
            ),
            4
        );
        assert_eq!(
            sum(
                &parse_input(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap(),
                Some("red")
            ),
            0
        );
        assert_eq!(sum(&parse_input(r#"[1,"red",5]"#).unwrap(), Some("red")), 6);
    }
}
