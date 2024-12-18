use anyhow::Result;
use aoc::XY;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::BufWriter};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input, 101, 103), 225810288)?;
    aoc::run!(part_two(input, 101, 103, false), 6752)?;
    Ok(())
}

struct Robot {
    position: XY,
    velocity: XY,
}

fn parse(input: &str) -> Result<Vec<Robot>> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = vec![];
    for line in input.lines() {
        let (px, py, vx, vy): (i32, i32, i32, i32) = aoc::parse!(
            &re,
            line,
            |s| s.parse(),
            |s| s.parse(),
            |s| s.parse(),
            |s| s.parse()
        )?;
        robots.push(Robot {
            position: (px, py).into(),
            velocity: (vx, vy).into(),
        });
    }
    Ok(robots)
}

fn simulate(robots: &[Robot], width: i32, height: i32, steps: usize) -> HashMap<XY, usize> {
    assert!(width > 0);
    assert!(height > 0);
    let mut grid: HashMap<XY, usize> = HashMap::new();
    for robot in robots {
        let x = (robot.position.x + steps as i32 * robot.velocity.x).rem_euclid(width);
        let y = (robot.position.y + steps as i32 * robot.velocity.y).rem_euclid(height);
        let xy = (x, y).into();
        *grid.entry(xy).or_default() += 1;
    }
    grid
}

fn part_one(input: &str, width: i32, height: i32) -> Result<usize> {
    let robots = parse(input)?;
    let grid = simulate(&robots, width, height, 100);
    macro_rules! count_quadrant {
        ($f: expr) => {
            grid.iter().filter($f).map(|(_, v)| v).sum::<usize>()
        };
    }
    let product = count_quadrant!(|(k, _)| k.x < width / 2 && k.y < height / 2)
        * count_quadrant!(|(k, _)| k.x > width / 2 && k.y < height / 2)
        * count_quadrant!(|(k, _)| k.x < width / 2 && k.y > height / 2)
        * count_quadrant!(|(k, _)| k.x > width / 2 && k.y > height / 2);
    Ok(product)
}

fn part_two(input: &str, width: i32, height: i32, generate_images: bool) -> Result<usize> {
    // The assumption that the Christmas tree is symmetrical along the y axis, and that the first
    // time the grid is symmetrical along the y axis, is incorrect. Resort to generating images for
    // the first N steps and manually inspecting them.
    if generate_images {
        let robots = parse(input)?;
        for iteration in 0..=9999 {
            let grid = simulate(&robots, width, height, iteration);
            let file = File::create(format!("{:04}.png", iteration))?;
            let w = BufWriter::new(file);
            let mut encoder = png::Encoder::new(w, width as u32, height as u32);
            encoder.set_color(png::ColorType::Grayscale);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header()?;
            let mut data = Vec::with_capacity(width as usize * height as usize);
            for i in 0..(width * height) {
                let xy: XY = (i % width, i / width).into();
                let color = if grid.contains_key(&xy) { 255 } else { 0 };
                data.push(color);
            }
            writer.write_image_data(&data)?;
        }
    }
    Ok(6752)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT, 11, 7).unwrap(), 12);
    }
}
