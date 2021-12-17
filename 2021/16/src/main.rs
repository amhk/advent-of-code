fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadArgument(String),
    NoMoreBits,
    Corrupt(&'static str),
}

#[derive(Debug, PartialEq, Clone)]
enum Bit {
    Zero,
    One,
}

#[derive(Debug)]
struct Scanner {
    bits: Vec<Bit>,
    position: usize,
}

impl Scanner {
    fn read(&mut self, n: usize) -> Result<u64, Error> {
        if n > u64::BITS as usize {
            return Err(Error::BadArgument(format!("{}", n)));
        }
        let mut value = 0;
        for i in 0..n {
            value = match self.bits.get(self.position + i).ok_or(Error::NoMoreBits)? {
                Bit::Zero => value << 1,
                Bit::One => (value << 1) | 0x0001,
            };
        }
        self.position += n;
        Ok(value)
    }
}

impl TryFrom<&str> for Scanner {
    type Error = Error;

    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        const B0: Bit = Bit::Zero;
        const B1: Bit = Bit::One;

        let mut bits = Vec::new();
        for ch in hex.chars() {
            match ch {
                '0' => bits.append(&mut vec![B0, B0, B0, B0]),
                '1' => bits.append(&mut vec![B0, B0, B0, B1]),
                '2' => bits.append(&mut vec![B0, B0, B1, B0]),
                '3' => bits.append(&mut vec![B0, B0, B1, B1]),
                '4' => bits.append(&mut vec![B0, B1, B0, B0]),
                '5' => bits.append(&mut vec![B0, B1, B0, B1]),
                '6' => bits.append(&mut vec![B0, B1, B1, B0]),
                '7' => bits.append(&mut vec![B0, B1, B1, B1]),
                '8' => bits.append(&mut vec![B1, B0, B0, B0]),
                '9' => bits.append(&mut vec![B1, B0, B0, B1]),
                'A' => bits.append(&mut vec![B1, B0, B1, B0]),
                'B' => bits.append(&mut vec![B1, B0, B1, B1]),
                'C' => bits.append(&mut vec![B1, B1, B0, B0]),
                'D' => bits.append(&mut vec![B1, B1, B0, B1]),
                'E' => bits.append(&mut vec![B1, B1, B1, B0]),
                'F' => bits.append(&mut vec![B1, B1, B1, B1]),
                _ => return Err(Error::BadArgument(format!("{}", ch))),
            }
        }
        Ok(Scanner { bits, position: 0 })
    }
}

#[derive(Debug)]
struct Parser {
    scanner: Scanner,
}

impl Parser {
    fn new(scanner: Scanner) -> Parser {
        Parser { scanner }
    }

    /*
     * Grammar
     *
     * contents := package Z*
     * package := package_header package_payload
     * packet_header := package_version package_type
     * package_version := VVV
     * package_type := TTT
     * package_payload := payload_literal | payload_operator
     * payload_literal := (1 PPPP)* 0 PPPP
     * payload_operator := 0 L{15} package+ | 1 N{11} package+
     */
    fn parse_package(&mut self) -> Result<ASTNode, Error> {
        let version = self.scanner.read(3)? as Version;
        let type_: PackageType = self.scanner.read(3)?.try_into()?;

        if type_ == PackageType::Constant {
            let mut value = 0;
            for _ in 0..(u64::BITS / 4) {
                let repeat = self.scanner.read(1)? == 1;
                value = (value << 4) | self.scanner.read(4)?;
                if !repeat {
                    return Ok(ASTNode::Constant(version, value));
                }
            }
            return Err(Error::Corrupt("constant exceeds 64 bits"));
        }

        let mut sub_packages = vec![];
        match self.scanner.read(1)? {
            0 => {
                let len = self.scanner.read(15)? as usize;
                let mut sub_parser = self.sub_parser(len);
                while sub_parser.scanner.position < len {
                    sub_packages.push(sub_parser.parse_package()?);
                }
            }
            1 => {
                let n = self.scanner.read(11)?;
                for _ in 0..n {
                    sub_packages.push(self.parse_package()?);
                }
            }
            _ => unreachable!(),
        }
        if sub_packages.is_empty() {
            return Err(Error::Corrupt("no children"));
        }

        let node = match type_ {
            PackageType::Sum => ASTNode::Sum(version, sub_packages),
            PackageType::Product => ASTNode::Product(version, sub_packages),
            PackageType::Minimum => ASTNode::Minimum(version, sub_packages),
            PackageType::Maximum => ASTNode::Maximum(version, sub_packages),
            PackageType::Constant => unreachable!(), // handled above
            PackageType::GreaterThan => {
                let child_right = sub_packages
                    .pop()
                    .ok_or(Error::Corrupt("missing left child"))?;
                let child_left = sub_packages
                    .pop()
                    .ok_or(Error::Corrupt("missing left child"))?;
                if !sub_packages.is_empty() {
                    return Err(Error::Corrupt("unexpected children"));
                }
                ASTNode::GreaterThan(version, Box::new(child_left), Box::new(child_right))
            }
            PackageType::LessThan => {
                let child_right = sub_packages
                    .pop()
                    .ok_or(Error::Corrupt("missing left child"))?;
                let child_left = sub_packages
                    .pop()
                    .ok_or(Error::Corrupt("missing left child"))?;
                if !sub_packages.is_empty() {
                    return Err(Error::Corrupt("unexpected children"));
                }
                ASTNode::LessThan(version, Box::new(child_left), Box::new(child_right))
            }
            PackageType::Equal => {
                let child_left = sub_packages
                    .pop()
                    .ok_or(Error::Corrupt("missing left child"))?;
                let child_right = sub_packages
                    .pop()
                    .ok_or(Error::Corrupt("missing left child"))?;
                if !sub_packages.is_empty() {
                    return Err(Error::Corrupt("unexpected children"));
                }
                ASTNode::Equal(version, Box::new(child_left), Box::new(child_right))
            }
        };
        Ok(node)
    }

    fn sub_parser(&mut self, len: usize) -> Parser {
        let sub_scanner = Scanner {
            bits: self.scanner.bits[self.scanner.position..self.scanner.position + len].to_vec(),
            position: 0,
        };
        self.scanner.position += len;
        Parser::new(sub_scanner)
    }
}

#[derive(Debug, PartialEq)]
enum PackageType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Constant = 4,
    GreaterThan = 5,
    LessThan = 6,
    Equal = 7,
}

impl TryFrom<u64> for PackageType {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Sum as u64 => Ok(Self::Sum),
            x if x == Self::Product as u64 => Ok(Self::Product),
            x if x == Self::Minimum as u64 => Ok(Self::Minimum),
            x if x == Self::Maximum as u64 => Ok(Self::Maximum),
            x if x == Self::Constant as u64 => Ok(Self::Constant),
            x if x == Self::GreaterThan as u64 => Ok(Self::GreaterThan),
            x if x == Self::LessThan as u64 => Ok(Self::LessThan),
            x if x == Self::Equal as u64 => Ok(Self::Equal),
            _ => Err(Error::BadArgument(format!(
                "failed to convert {} to PackageType",
                value
            ))),
        }
    }
}

type Version = u64;

#[derive(Debug, PartialEq)]
enum ASTNode {
    Sum(Version, Vec<ASTNode>),
    Product(Version, Vec<ASTNode>),
    Minimum(Version, Vec<ASTNode>),
    Maximum(Version, Vec<ASTNode>),
    Constant(Version, u64),
    GreaterThan(Version, Box<ASTNode>, Box<ASTNode>),
    LessThan(Version, Box<ASTNode>, Box<ASTNode>),
    Equal(Version, Box<ASTNode>, Box<ASTNode>),
}

impl ASTNode {
    fn visit_versions(&self) -> Result<u64, Error> {
        match self {
            ASTNode::Sum(v, children) => {
                let mut sum = *v;
                for child in children.iter() {
                    sum += child.visit_versions()?;
                }
                Ok(sum)
            }
            ASTNode::Product(v, children) => {
                let mut sum = *v;
                for child in children.iter() {
                    sum += child.visit_versions()?;
                }
                Ok(sum)
            }
            ASTNode::Minimum(v, children) => {
                let mut sum = *v;
                for child in children.iter() {
                    sum += child.visit_versions()?;
                }
                Ok(sum)
            }
            ASTNode::Maximum(v, children) => {
                let mut sum = *v;
                for child in children.iter() {
                    sum += child.visit_versions()?;
                }
                Ok(sum)
            }
            ASTNode::Constant(v, _) => Ok(*v),
            ASTNode::GreaterThan(v, left, right) => {
                Ok(v + left.visit_versions()? + right.visit_versions()?)
            }
            ASTNode::LessThan(v, left, right) => {
                Ok(v + left.visit_versions()? + right.visit_versions()?)
            }
            ASTNode::Equal(v, left, right) => {
                Ok(v + left.visit_versions()? + right.visit_versions()?)
            }
        }
    }

    fn eval(&self) -> Result<u64, Error> {
        match self {
            ASTNode::Sum(_, children) => {
                let mut sum = 0;
                for child in children.iter() {
                    sum += child.eval()?;
                }
                Ok(sum)
            }
            ASTNode::Product(_, children) => {
                let mut product = 1;
                for child in children.iter() {
                    product *= child.eval()?;
                }
                Ok(product)
            }
            ASTNode::Minimum(_, children) => {
                let x: Result<Vec<_>, _> = children.iter().map(|child| child.eval()).collect();
                Ok(*x?.iter().min().unwrap())
            }
            ASTNode::Maximum(_, children) => {
                let x: Result<Vec<_>, _> = children.iter().map(|child| child.eval()).collect();
                Ok(*x?.iter().max().unwrap())
            }
            ASTNode::Constant(_, value) => Ok(*value),
            ASTNode::GreaterThan(_, left, right) => {
                if left.eval()? > right.eval()? {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            ASTNode::LessThan(_, left, right) => {
                if left.eval()? < right.eval()? {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            ASTNode::Equal(_, left, right) => {
                if left.eval()? == right.eval()? {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
        }
    }
}

fn part_one(input: &str) -> Result<u64, Error> {
    let scanner: Scanner = input.trim().try_into()?;
    let mut parser = Parser::new(scanner);
    let pkg = parser.parse_package()?;
    pkg.visit_versions()
}

fn part_two(input: &str) -> Result<u64, Error> {
    let scanner: Scanner = input.trim().try_into()?;
    let mut parser = Parser::new(scanner);
    let pkg = parser.parse_package()?;
    pkg.eval()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner() {
        let scanner: Scanner = "0F".try_into().unwrap();
        assert_eq!(scanner.position, 0);
        assert_eq!(
            scanner.bits,
            [
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One
            ]
        );
    }

    #[test]
    fn test_parse_package_literal_value() {
        let scanner: Scanner = "D2FE28".try_into().unwrap();
        let mut parser = Parser::new(scanner);
        let pkg = parser.parse_package().unwrap();
        assert_eq!(pkg.visit_versions(), Ok(6));
    }

    #[test]
    fn test_parse_constant_type_0() {
        let scanner: Scanner = "38006F45291200".try_into().unwrap();
        let mut parser = Parser::new(scanner);
        let pkg = parser.parse_package().unwrap();
        assert_eq!(pkg.visit_versions(), Ok(1 + 6 + 2));
    }

    #[test]
    fn test_parse_constant_type_1() {
        let scanner: Scanner = "EE00D40C823060".try_into().unwrap();
        let mut parser = Parser::new(scanner);
        let pkg = parser.parse_package().unwrap();
        assert_eq!(pkg.visit_versions(), Ok(7 + 2 + 4 + 1));
    }

    #[test]
    fn test_eval_sum() {
        let scanner: Scanner = "C200B40A82".try_into().unwrap();
        let mut parser = Parser::new(scanner);
        let pkg = parser.parse_package().unwrap();
        assert_eq!(pkg.eval(), Ok(3));
    }

    #[test]
    fn test_eval_product() {
        let scanner: Scanner = "04005AC33890".try_into().unwrap();
        let mut parser = Parser::new(scanner);
        let pkg = parser.parse_package().unwrap();
        assert_eq!(pkg.eval(), Ok(54));
    }

    #[test]
    fn test_eval_minimum() {
        let scanner: Scanner = "880086C3E88112".try_into().unwrap();
        let mut parser = Parser::new(scanner);
        let pkg = parser.parse_package().unwrap();
        assert_eq!(pkg.eval(), Ok(7));
    }

    #[test]
    fn test_eval_maximum() {
        let scanner: Scanner = "CE00C43D881120".try_into().unwrap();
        let mut parser = Parser::new(scanner);
        let pkg = parser.parse_package().unwrap();
        assert_eq!(pkg.eval(), Ok(9));
    }

    #[test]
    fn test_eval_greater_than() {
        let scanner: Scanner = "D8005AC2A8F0".try_into().unwrap();
        let mut parser = Parser::new(scanner);
        let pkg = parser.parse_package().unwrap();
        assert_eq!(pkg.eval(), Ok(1));
    }

    #[test]
    fn test_eval_less_than() {
        let scanner: Scanner = "F600BC2D8F".try_into().unwrap();
        let mut parser = Parser::new(scanner);
        let pkg = parser.parse_package().unwrap();
        assert_eq!(pkg.eval(), Ok(0));
    }

    #[test]
    fn test_eval_equal() {
        let scanner: Scanner = "9C005AC2F8F0".try_into().unwrap();
        let mut parser = Parser::new(scanner);
        let pkg = parser.parse_package().unwrap();
        assert_eq!(pkg.eval(), Ok(0));
    }

    #[test]
    fn test_eval() {
        let scanner: Scanner = "9C0141080250320F1802104A08".try_into().unwrap();
        let mut parser = Parser::new(scanner);
        let pkg = parser.parse_package().unwrap();
        assert_eq!(pkg.eval(), Ok(1));
    }
}
