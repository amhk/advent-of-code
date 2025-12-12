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
    EmptyStack,
}

#[derive(Debug)]
enum Node {
    Node(Box<Node>, Box<Node>),
    Leaf(u32),
}

impl Node {
    // ---- split ----
    fn split(&mut self) {
        match *self {
            Node::Node(_, _) => panic!(),
            Node::Leaf(value) => {
                let left = Box::new(Node::Leaf(value / 2));
                let right = Box::new(Node::Leaf(value.div_ceil(2)));
                *self = Node::Node(left, right);
            }
        }
    }

    pub fn try_split(&mut self) -> bool {
        match self {
            Node::Node(left, right) => left.try_split() || right.try_split(),
            Node::Leaf(value) => {
                if *value >= 10 {
                    self.split();
                    return true;
                }
                false
            }
        }
    }

    pub fn try_explode(&mut self) -> bool {
        #[derive(Debug, PartialEq)]
        enum State {
            SearchingForPairToExplode,
            SearchingForLeafToAddTo,
            Done,
        }
        #[derive(Debug)]
        struct Context<'a> {
            depth: usize,
            state: State,
            left_of_exploding: Option<&'a Node>,
            exploding: Option<&'a Node>,
            right_of_exploding: Option<&'a Node>,
        }
        fn explode_inner<'a>(this: &'a Node, mut ctx: Context<'a>) -> Context<'a> {
            if ctx.state == State::Done {
                return ctx;
            }
            ctx.depth += 1;
            match this {
                Node::Node(left, right) => match ctx.state {
                    State::SearchingForPairToExplode => {
                        if ctx.depth == 5 {
                            ctx.state = State::SearchingForLeafToAddTo;
                            ctx.exploding = Some(this);
                        } else {
                            ctx = explode_inner(left, ctx);
                            ctx = explode_inner(right, ctx);
                        }
                    }
                    State::SearchingForLeafToAddTo => {
                        ctx = explode_inner(left, ctx);
                        ctx = explode_inner(right, ctx);
                    }
                    State::Done => unreachable!(),
                },
                Node::Leaf(_) => match ctx.state {
                    State::SearchingForPairToExplode => {
                        ctx.left_of_exploding = Some(this);
                    }
                    State::SearchingForLeafToAddTo => {
                        ctx.right_of_exploding = Some(this);
                        ctx.state = State::Done;
                    }
                    State::Done => unreachable!(),
                },
            }
            ctx.depth -= 1;
            ctx
        }
        let ctx = explode_inner(
            self,
            Context {
                depth: 0,
                state: State::SearchingForPairToExplode,
                left_of_exploding: None,
                exploding: None,
                right_of_exploding: None,
            },
        );

        #[allow(clippy::transmute_ptr_to_ref)]
        if let Some(exploding) = ctx.exploding {
            let (left, right) = match exploding {
                Node::Node(left, right) => (left, right),
                Node::Leaf(_) => unreachable!(),
            };
            let left = match **left {
                Node::Node(_, _) => unreachable!(),
                Node::Leaf(value) => value,
            };
            let right = match **right {
                Node::Node(_, _) => unreachable!(),
                Node::Leaf(value) => value,
            };

            if let Some(left_of_exploding) = ctx.left_of_exploding {
                let ptr: *const Node = left_of_exploding;
                let left_of_exploding =
                    unsafe { std::mem::transmute::<*const Node, &mut Node>(ptr) };
                if let Node::Leaf(value) = left_of_exploding {
                    *value += left;
                }
            }

            if let Some(right_of_exploding) = ctx.right_of_exploding {
                let ptr: *const Node = right_of_exploding;
                let right_of_exploding =
                    unsafe { std::mem::transmute::<*const Node, &mut Node>(ptr) };
                if let Node::Leaf(value) = right_of_exploding {
                    *value += right;
                }
            }

            let ptr: *const Node = exploding;
            let exploding = unsafe { std::mem::transmute::<*const Node, &mut Node>(ptr) };
            *exploding = Node::Leaf(0);
            return true;
        }
        false
    }

    fn add(self, other: Box<Node>) -> Box<Node> {
        Box::new(Node::Node(Box::new(self), other))
    }

    fn reduce(&mut self) {
        loop {
            if self.try_explode() {
                continue;
            }
            if self.try_split() {
                continue;
            }
            break;
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Node::Node(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
            Node::Leaf(value) => *value,
        }
    }
}

impl TryFrom<&str> for Box<Node> {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        #[derive(Debug)]
        enum Token {
            Constant(u32),
            OpeningBrace,
        }

        fn graph_from_tokens(tokens: &mut Vec<Token>) -> Result<Box<Node>, Error> {
            match tokens.pop() {
                None => Err(Error::EmptyStack),
                Some(Token::Constant(value)) => Ok(Box::new(Node::Leaf(value))),
                Some(Token::OpeningBrace) => {
                    let right = graph_from_tokens(tokens)?;
                    let left = graph_from_tokens(tokens)?;
                    Ok(Box::new(Node::Node(left, right)))
                }
            }
        }

        let mut stack = vec![];
        let mut output = vec![];
        let mut chars: Vec<_> = value.trim().chars().rev().collect();
        while let Some(ch) = chars.pop() {
            match ch {
                '[' => stack.push(Token::OpeningBrace),
                #[allow(clippy::never_loop)]
                ']' => loop {
                    match stack.pop() {
                        None => return Err(Error::BadInput),
                        Some(Token::Constant(_)) => unreachable!(),
                        Some(Token::OpeningBrace) => {
                            output.push(Token::OpeningBrace);
                            break;
                        }
                    }
                },
                ',' => {} // no-op
                x if x.is_ascii_digit() => {
                    let mut s = format!("{}", x);
                    while let Some(next_ch) = chars.last() {
                        if next_ch.is_ascii_digit() {
                            s.push(chars.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    let value: u32 = s.parse().unwrap();
                    output.push(Token::Constant(value));
                }
                _ => return Err(Error::BadInput),
            }
        }

        let graph = graph_from_tokens(&mut output)?;
        if !output.is_empty() {
            return Err(Error::BadInput);
        }
        Ok(graph)
    }
}

impl std::fmt::Display for Box<Node> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &**self {
            Node::Node(left, right) => {
                write!(f, "[{},{}]", left, right)
            }
            Node::Leaf(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

#[allow(clippy::vec_box)]
fn parse_input(input: &str) -> Result<Vec<Box<Node>>, Error> {
    let mut nodes = vec![];
    for line in input.lines() {
        nodes.push(line.try_into()?);
    }
    Ok(nodes)
}

fn part_one(input: &str) -> Result<u32, Error> {
    let mut nodes: Vec<_> = parse_input(input)?.into_iter().rev().collect();
    let mut node = nodes.pop().ok_or(Error::BadInput)?;
    while let Some(other) = nodes.pop() {
        node = node.add(other);
        node.reduce();
    }
    Ok(node.magnitude())
}

fn part_two(input: &str) -> Result<u32, Error> {
    let mut magnitudes = vec![];
    let lines: Vec<_> = input.lines().collect();
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                continue;
            }

            for pair in [(i, j), (j, i)] {
                let a: Box<Node> = lines[pair.0].try_into()?;
                let b: Box<Node> = lines[pair.1].try_into()?;
                let mut c = a.add(b);
                c.reduce();
                magnitudes.push(c.magnitude());
            }
        }
    }
    let max = magnitudes.iter().max().ok_or(Error::BadInput)?;
    Ok(*max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_node_try_from() {
        macro_rules! assert_encode_decode {
            ($str:expr) => {
                let node: Box<Node> = $str.try_into().unwrap();
                assert_eq!(format!("{}", node), $str);
            };
        }
        assert_encode_decode!("[1,2]");
        assert_encode_decode!("[[1,2],3]");
        assert_encode_decode!("[9,[8,7]]");
        assert_encode_decode!("[[1,9],[8,5]]");
        assert_encode_decode!("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
        assert_encode_decode!("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        assert_encode_decode!("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    }

    #[test]
    fn test_split() {
        let mut node: Box<Node> = "[[[[0,7],4],[15,[0,13]]],[1,1]]".try_into().unwrap();
        assert!(node.try_split());
        assert_eq!(format!("{}", node), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    }

    #[test]
    fn test_explode() {
        macro_rules! assert_explode {
            ($before:expr, $after:expr) => {
                let mut node: Box<Node> = $before.try_into().unwrap();
                assert!(node.try_explode());
                assert_eq!(format!("{}", node), $after);
            };
        }
        assert_explode!("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        assert_explode!("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        assert_explode!("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        assert_explode!(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
        );
        assert_explode!(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
        );
    }

    #[test]
    fn test_add() {
        let a: Box<Node> = "[[[[4,3],4],4],[7,[[8,4],9]]]".try_into().unwrap();
        let b: Box<Node> = "[1,1]".try_into().unwrap();
        assert_eq!(
            format!("{}", a.add(b)),
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"
        );
    }

    #[test]
    fn test_reduce() {
        let mut node: Box<Node> = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".try_into().unwrap();
        node.reduce();
        assert_eq!(format!("{}", node), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_sum() {
        let mut nodes: Vec<_> = parse_input(INPUT).unwrap().into_iter().rev().collect();
        let mut node = nodes.pop().unwrap();
        while let Some(other) = nodes.pop() {
            node = node.add(other);
            node.reduce();
        }
        assert_eq!(
            format!("{}", node),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );
    }

    #[test]
    fn test_magnitude() {
        let node: Box<Node> = "[[9,1],[1,9]]".try_into().unwrap();
        assert_eq!(node.magnitude(), 129);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(4140));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(3993));
    }
}
