use anyhow::{anyhow, ensure};
use regex::Regex;
use std::fmt::Display;

use crate::{BoundingBox, XY};

const REGEX_NOT_FOUND: &str = "regex not found in \"{haystack}\"";

pub fn parse1<'h, A, FA, EA>(regex: &Regex, haystack: &'h str, to_a: FA) -> anyhow::Result<A>
where
    FA: Fn(&'h str) -> Result<A, EA>,
    EA: Display + Send + Sync,
{
    let (_, [a]) = regex
        .captures(haystack)
        .map(|caps| caps.extract())
        .ok_or_else(|| anyhow!(REGEX_NOT_FOUND))?;
    to_a(a).map_err(|err| anyhow!("group 1: \"{a}\": {err}"))
}

pub fn parse2<'h, A, FA, EA, B, FB, EB>(
    regex: &Regex,
    haystack: &'h str,
    to_a: FA,
    to_b: FB,
) -> anyhow::Result<(A, B)>
where
    FA: Fn(&'h str) -> Result<A, EA>,
    FB: Fn(&'h str) -> Result<B, EB>,
    EA: Display + Send + Sync,
    EB: Display + Send + Sync,
{
    let (_, [a, b]) = regex
        .captures(haystack)
        .map(|caps| caps.extract())
        .ok_or_else(|| anyhow!(REGEX_NOT_FOUND))?;
    Ok((
        to_a(a).map_err(|err| anyhow!("group 1: \"{a}\": {err}"))?,
        to_b(b).map_err(|err| anyhow!("group 2: \"{b}\": {err}"))?,
    ))
}

pub fn parse3<'h, A, FA, EA, B, FB, EB, C, FC, EC>(
    regex: &Regex,
    haystack: &'h str,
    to_a: FA,
    to_b: FB,
    to_c: FC,
) -> anyhow::Result<(A, B, C)>
where
    FA: Fn(&'h str) -> Result<A, EA>,
    FB: Fn(&'h str) -> Result<B, EB>,
    FC: Fn(&'h str) -> Result<C, EC>,
    EA: Display + Send + Sync,
    EB: Display + Send + Sync,
    EC: Display + Send + Sync,
{
    let (_, [a, b, c]) = regex
        .captures(haystack)
        .map(|caps| caps.extract())
        .ok_or_else(|| anyhow!(REGEX_NOT_FOUND))?;
    Ok((
        to_a(a).map_err(|err| anyhow!("group 1: \"{a}\": {err}"))?,
        to_b(b).map_err(|err| anyhow!("group 2: \"{b}\": {err}"))?,
        to_c(c).map_err(|err| anyhow!("group 3: \"{c}\": {err}"))?,
    ))
}

pub fn parse4<'h, A, FA, EA, B, FB, EB, C, FC, EC, D, FD, ED>(
    regex: &Regex,
    haystack: &'h str,
    to_a: FA,
    to_b: FB,
    to_c: FC,
    to_d: FD,
) -> anyhow::Result<(A, B, C, D)>
where
    FA: Fn(&'h str) -> Result<A, EA>,
    FB: Fn(&'h str) -> Result<B, EB>,
    FC: Fn(&'h str) -> Result<C, EC>,
    FD: Fn(&'h str) -> Result<D, ED>,
    EA: Display + Send + Sync,
    EB: Display + Send + Sync,
    EC: Display + Send + Sync,
    ED: Display + Send + Sync,
{
    let (_, [a, b, c, d]) = regex
        .captures(haystack)
        .map(|caps| caps.extract())
        .ok_or_else(|| anyhow!(REGEX_NOT_FOUND))?;
    Ok((
        to_a(a).map_err(|err| anyhow!("group 1: \"{a}\": {err}"))?,
        to_b(b).map_err(|err| anyhow!("group 2: \"{b}\": {err}"))?,
        to_c(c).map_err(|err| anyhow!("group 3: \"{c}\": {err}"))?,
        to_d(d).map_err(|err| anyhow!("group 4: \"{d}\": {err}"))?,
    ))
}

pub fn parse5<'h, A, FA, EA, B, FB, EB, C, FC, EC, D, FD, ED, E, FE, EE>(
    regex: &Regex,
    haystack: &'h str,
    to_a: FA,
    to_b: FB,
    to_c: FC,
    to_d: FD,
    to_e: FE,
) -> anyhow::Result<(A, B, C, D, E)>
where
    FA: Fn(&'h str) -> Result<A, EA>,
    FB: Fn(&'h str) -> Result<B, EB>,
    FC: Fn(&'h str) -> Result<C, EC>,
    FD: Fn(&'h str) -> Result<D, ED>,
    FE: Fn(&'h str) -> Result<E, EE>,
    EA: Display + Send + Sync,
    EB: Display + Send + Sync,
    EC: Display + Send + Sync,
    ED: Display + Send + Sync,
    EE: Display + Send + Sync,
{
    let (_, [a, b, c, d, e]) = regex
        .captures(haystack)
        .map(|caps| caps.extract())
        .ok_or_else(|| anyhow!(REGEX_NOT_FOUND))?;
    Ok((
        to_a(a).map_err(|err| anyhow!("group 1: \"{a}\": {err}"))?,
        to_b(b).map_err(|err| anyhow!("group 2: \"{b}\": {err}"))?,
        to_c(c).map_err(|err| anyhow!("group 3: \"{c}\": {err}"))?,
        to_d(d).map_err(|err| anyhow!("group 4: \"{d}\": {err}"))?,
        to_e(e).map_err(|err| anyhow!("group 5: \"{e}\": {err}"))?,
    ))
}

#[allow(clippy::too_many_arguments)]
pub fn parse6<'h, A, FA, EA, B, FB, EB, C, FC, EC, D, FD, ED, E, FE, EE, F, FF, EF>(
    regex: &Regex,
    haystack: &'h str,
    to_a: FA,
    to_b: FB,
    to_c: FC,
    to_d: FD,
    to_e: FE,
    to_f: FF,
) -> anyhow::Result<(A, B, C, D, E, F)>
where
    FA: Fn(&'h str) -> Result<A, EA>,
    FB: Fn(&'h str) -> Result<B, EB>,
    FC: Fn(&'h str) -> Result<C, EC>,
    FD: Fn(&'h str) -> Result<D, ED>,
    FE: Fn(&'h str) -> Result<E, EE>,
    FF: Fn(&'h str) -> Result<F, EF>,
    EA: Display + Send + Sync,
    EB: Display + Send + Sync,
    EC: Display + Send + Sync,
    ED: Display + Send + Sync,
    EE: Display + Send + Sync,
    EF: Display + Send + Sync,
{
    let (_, [a, b, c, d, e, f]) = regex
        .captures(haystack)
        .map(|caps| caps.extract())
        .ok_or_else(|| anyhow!(REGEX_NOT_FOUND))?;
    Ok((
        to_a(a).map_err(|err| anyhow!("group 1: \"{a}\": {err}"))?,
        to_b(b).map_err(|err| anyhow!("group 2: \"{b}\": {err}"))?,
        to_c(c).map_err(|err| anyhow!("group 3: \"{c}\": {err}"))?,
        to_d(d).map_err(|err| anyhow!("group 4: \"{d}\": {err}"))?,
        to_e(e).map_err(|err| anyhow!("group 5: \"{e}\": {err}"))?,
        to_f(f).map_err(|err| anyhow!("group 6: \"{f}\": {err}"))?,
    ))
}

#[allow(clippy::too_many_arguments)]
pub fn parse7<'h, A, FA, EA, B, FB, EB, C, FC, EC, D, FD, ED, E, FE, EE, F, FF, EF, G, FG, EG>(
    regex: &Regex,
    haystack: &'h str,
    to_a: FA,
    to_b: FB,
    to_c: FC,
    to_d: FD,
    to_e: FE,
    to_f: FF,
    to_g: FG,
) -> anyhow::Result<(A, B, C, D, E, F, G)>
where
    FA: Fn(&'h str) -> Result<A, EA>,
    FB: Fn(&'h str) -> Result<B, EB>,
    FC: Fn(&'h str) -> Result<C, EC>,
    FD: Fn(&'h str) -> Result<D, ED>,
    FE: Fn(&'h str) -> Result<E, EE>,
    FF: Fn(&'h str) -> Result<F, EF>,
    FG: Fn(&'h str) -> Result<G, EG>,
    EA: Display + Send + Sync,
    EB: Display + Send + Sync,
    EC: Display + Send + Sync,
    ED: Display + Send + Sync,
    EE: Display + Send + Sync,
    EF: Display + Send + Sync,
    EG: Display + Send + Sync,
{
    let (_, [a, b, c, d, e, f, g]) = regex
        .captures(haystack)
        .map(|caps| caps.extract())
        .ok_or_else(|| anyhow!(REGEX_NOT_FOUND))?;
    Ok((
        to_a(a).map_err(|err| anyhow!("group 1: \"{a}\": {err}"))?,
        to_b(b).map_err(|err| anyhow!("group 2: \"{b}\": {err}"))?,
        to_c(c).map_err(|err| anyhow!("group 3: \"{c}\": {err}"))?,
        to_d(d).map_err(|err| anyhow!("group 4: \"{d}\": {err}"))?,
        to_e(e).map_err(|err| anyhow!("group 5: \"{e}\": {err}"))?,
        to_f(f).map_err(|err| anyhow!("group 6: \"{f}\": {err}"))?,
        to_g(g).map_err(|err| anyhow!("group 7: \"{g}\": {err}"))?,
    ))
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn parse8<
    'h,
    A,
    FA,
    EA,
    B,
    FB,
    EB,
    C,
    FC,
    EC,
    D,
    FD,
    ED,
    E,
    FE,
    EE,
    F,
    FF,
    EF,
    G,
    FG,
    EG,
    H,
    FH,
    EH,
>(
    regex: &Regex,
    haystack: &'h str,
    to_a: FA,
    to_b: FB,
    to_c: FC,
    to_d: FD,
    to_e: FE,
    to_f: FF,
    to_g: FG,
    to_h: FH,
) -> anyhow::Result<(A, B, C, D, E, F, G, H)>
where
    FA: Fn(&'h str) -> Result<A, EA>,
    FB: Fn(&'h str) -> Result<B, EB>,
    FC: Fn(&'h str) -> Result<C, EC>,
    FD: Fn(&'h str) -> Result<D, ED>,
    FE: Fn(&'h str) -> Result<E, EE>,
    FF: Fn(&'h str) -> Result<F, EF>,
    FG: Fn(&'h str) -> Result<G, EG>,
    FH: Fn(&'h str) -> Result<H, EH>,
    EA: Display + Send + Sync,
    EB: Display + Send + Sync,
    EC: Display + Send + Sync,
    ED: Display + Send + Sync,
    EE: Display + Send + Sync,
    EF: Display + Send + Sync,
    EG: Display + Send + Sync,
    EH: Display + Send + Sync,
{
    let (_, [a, b, c, d, e, f, g, h]) = regex
        .captures(haystack)
        .map(|caps| caps.extract())
        .ok_or_else(|| anyhow!(REGEX_NOT_FOUND))?;
    Ok((
        to_a(a).map_err(|err| anyhow!("group 1: \"{a}\": {err}"))?,
        to_b(b).map_err(|err| anyhow!("group 2: \"{b}\": {err}"))?,
        to_c(c).map_err(|err| anyhow!("group 3: \"{c}\": {err}"))?,
        to_d(d).map_err(|err| anyhow!("group 4: \"{d}\": {err}"))?,
        to_e(e).map_err(|err| anyhow!("group 5: \"{e}\": {err}"))?,
        to_f(f).map_err(|err| anyhow!("group 6: \"{f}\": {err}"))?,
        to_g(g).map_err(|err| anyhow!("group 7: \"{g}\": {err}"))?,
        to_h(h).map_err(|err| anyhow!("group 8: \"{h}\": {err}"))?,
    ))
}

/// Parse an AOC input grid. These always look like this:
///
/// ..A..
/// A....
/// ..Ax.
/// .x...
///
/// For each character (that is not a newline), call f with the character's XY coordinate and
/// character value.
///
/// Returns the bounding box of the grid.
pub fn parse_grid<F>(input: &str, mut f: F) -> anyhow::Result<BoundingBox>
where
    F: FnMut(XY, char) -> anyhow::Result<()>,
{
    let input = input.trim();
    if input.is_empty() {
        return Ok(BoundingBox::new((0, 0).into(), (0, 0).into()));
    }
    let max_x = input.lines().next().expect("non-empty input").len();
    for line in input.lines() {
        ensure!(max_x == line.len());
    }
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for ch in input.chars() {
        if ch == '\n' {
            x = 0;
            y += 1;
        } else {
            f((x, y).into(), ch)?;
            x += 1;
        }
    }
    Ok(BoundingBox::new(
        (0, 0).into(),
        (max_x as i32 - 1, y).into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid() {
        let input = "ab\ncd\nef\n";
        let bb = parse_grid(&input, |xy, ch| {
            let expected = match ch {
                'a' => (0, 0).into(),
                'b' => (1, 0).into(),
                'c' => (0, 1).into(),
                'd' => (1, 1).into(),
                'e' => (0, 2).into(),
                'f' => (1, 2).into(),
                _ => panic!("unexpected {:?} {ch}", xy),
            };
            assert_eq!(xy, expected);
            Ok(())
        })
        .unwrap();
        assert_eq!(bb, BoundingBox::new((0, 0).into(), (1, 2).into()));
    }

    #[test]
    fn test_parse_grid_empty() {
        let input = "";
        let bb = parse_grid(&input, |_, _| Ok(())).unwrap();
        assert_eq!(bb, BoundingBox::new((0, 0).into(), (0, 0).into()));
    }

    #[test]
    fn test_parse_grid_different_length_lines() {
        let input = "..\n...\n";
        assert!(parse_grid(&input, |_, _| Ok(())).is_err());
    }

    #[test]
    fn test_parse_grid_trailing_newline_does_not_matter() {
        let input = "...\n...\n...\n...\n";
        let bb1 = parse_grid(&input, |_, _| Ok(())).unwrap();
        let bb2 = parse_grid(&input.trim(), |_, _| Ok(())).unwrap();
        assert_eq!(bb1, bb2);
    }
}
