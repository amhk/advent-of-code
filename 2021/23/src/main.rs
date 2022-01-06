use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

fn main() {
    let input1 = include_str!("input-part-one.txt");
    let input2 = include_str!("input-part-two.txt");

    let answer = part_one(input1).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input2).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
    UnknownChar(char),
    NoSolution,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Piece {
    Empty,
    A,
    B,
    C,
    D,
}

impl TryFrom<char> for Piece {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            _ => Err(Error::UnknownChar(value)),
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::A => 'A',
                Self::B => 'B',
                Self::C => 'C',
                Self::D => 'D',
            }
        )
    }
}

type SquareId = usize;

const X0: SquareId = 0;
const X1: SquareId = 1;
const X2: SquareId = 2;
const X3: SquareId = 3;
const X4: SquareId = 4;
const X5: SquareId = 5;
const X6: SquareId = 6;
const A3: SquareId = 7;
const B3: SquareId = 8;
const C3: SquareId = 9;
const D3: SquareId = 10;
const A2: SquareId = 11;
const B2: SquareId = 12;
const C2: SquareId = 13;
const D2: SquareId = 14;
const A1: SquareId = 15;
const B1: SquareId = 16;
const C1: SquareId = 17;
const D1: SquareId = 18;
const A0: SquareId = 19;
const B0: SquareId = 20;
const C0: SquareId = 21;
const D0: SquareId = 22;

const ALL_SQUARES: [SquareId; 23] = [
    X0, X1, X2, X3, X4, X5, X6, A3, B3, C3, D3, A2, B2, C2, D2, A1, B1, C1, D1, A0, B0, C0, D0,
];
const X_SQUARES: [SquareId; 7] = [X0, X1, X2, X3, X4, X5, X6];
const HOME_SQUARES_A: [SquareId; 4] = [A3, A2, A1, A0];
const HOME_SQUARES_B: [SquareId; 4] = [B3, B2, B1, B0];
const HOME_SQUARES_C: [SquareId; 4] = [C3, C2, C1, C0];
const HOME_SQUARES_D: [SquareId; 4] = [D3, D2, D1, D0];

lazy_static! {
    static ref POSSIBLE_MOVES: [Vec<(Vec<SquareId>, usize)>; 23] = [
        // X0
        vec![
            (vec![X1, A3], 3),
            (vec![X1, X2, B3], 5),
            (vec![X1, X2, X3, C3], 7),
            (vec![X1, X2, X3, X4, D3], 9),
        ],

        // X1
        vec![
            (vec![A3], 2),
            (vec![X2, B3], 4),
            (vec![X2, X3, C3], 6),
            (vec![X2, X3, X4, D3], 8),
        ],

        // X2
        vec![
            (vec![A3], 2),
            (vec![B3], 2),
            (vec![X3, C3], 4),
            (vec![X3, X4, D3], 6),
        ],

        // X3
        vec![
            (vec![X2, A3], 4),
            (vec![B3], 2),
            (vec![C3], 2),
            (vec![X4, D3], 4),
        ],

        // X4
        vec![
            (vec![X3, X2, A3], 6),
            (vec![X3, B3], 4),
            (vec![C3], 2),
            (vec![D3], 2),
        ],

        // X5
        vec![
            (vec![X4, X3, X2, A3], 8),
            (vec![X4, X3, B3], 6),
            (vec![X4, C3], 4),
            (vec![D3], 2),
        ],

        // X6
        vec![
            (vec![X5, X4, X3, X2, A3], 9),
            (vec![X5, X4, X3, B3], 7),
            (vec![X5, X4, C3], 5),
            (vec![X5, D3], 3),
        ],

        // A3
        vec![
            (vec![X1, X0], 3),
            (vec![X1], 2),
            (vec![X2], 2),
            (vec![X2, X3], 4),
            (vec![X2, X3, X4], 6),
            (vec![X2, X3, X4, X5], 8),
            (vec![X2, X3, X4, X5, X6], 9),
            (vec![A2], 1),
        ],

        // B3
        vec![
            (vec![X2, X1, X0], 5),
            (vec![X2, X1], 4),
            (vec![X2], 2),
            (vec![X3], 2),
            (vec![X3, X4], 4),
            (vec![X3, X4, X5], 6),
            (vec![X3, X4, X5, X6], 7),
            (vec![B2], 1),
        ],

        // C3
        vec![
            (vec![X3, X2, X1, X0], 7),
            (vec![X3, X2, X1], 6),
            (vec![X3, X2], 4),
            (vec![X3], 2),
            (vec![X4], 2),
            (vec![X4, X5], 4),
            (vec![X4, X5, X6], 5),
            (vec![C2], 1),
        ],

        // D3
        vec![
            (vec![X4, X3, X2, X1, X0], 9),
            (vec![X4, X3, X2, X1], 8),
            (vec![X4, X3, X2], 6),
            (vec![X4, X3], 4),
            (vec![X4], 2),
            (vec![X5], 2),
            (vec![X5, X6], 3),
            (vec![D2], 1),
        ],

        // A2
        vec![
            (vec![A1], 1),
            (vec![A3], 1),
        ],

        // B2
        vec![
            (vec![B1], 1),
            (vec![B3], 1),
        ],

        // C2
        vec![
            (vec![C1], 1),
            (vec![C3], 1),
        ],

        // D2
        vec![
            (vec![D1], 1),
            (vec![D3], 1),
        ],

        // A1
        vec![
            (vec![A0], 1),
            (vec![A2], 1),
        ],

        // B1
        vec![
            (vec![B0], 1),
            (vec![B2], 1),
        ],

        // C1
        vec![
            (vec![C0], 1),
            (vec![C2], 1),
        ],

        // D1
        vec![
            (vec![D0], 1),
            (vec![D2], 1),
        ],

        // A0
        vec![
            (vec![A1], 1),
        ],

        // B0
        vec![
            (vec![B1], 1),
        ],

        // C0
        vec![
            (vec![C1], 1),
        ],

        // D0
        vec![
            (vec![D1], 1),
        ],
    ];
}

/*
 * #############
 * #...........#      x0 x1 .. x2 .. x3 .. x4 .. x5 x6
 * ###A#B#C#D###  <=>       a3    b3    c3    d3
 *   #A#B#C#D#              a2    b2    c2    d2
 *   #A#B#C#D#              a1    b1    c1    d1
 *   #A#B#C#D#              a0    b0    c0    d0
 *   #########
 */
#[derive(PartialEq, Eq, Hash, Clone)]
struct Board {
    pieces: [Piece; 23],
}

const BOARD_SOLVED: Board = Board {
    pieces: [
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::A,
        Piece::B,
        Piece::C,
        Piece::D,
        Piece::A,
        Piece::B,
        Piece::C,
        Piece::D,
        Piece::A,
        Piece::B,
        Piece::C,
        Piece::D,
        Piece::A,
        Piece::B,
        Piece::C,
        Piece::D,
    ],
};

#[derive(Debug)]
struct Move {
    board: Board,
    cost: usize,
}

impl Move {
    fn score(&self) -> usize {
        fn score_home(board: &Board, piece: Piece) -> usize {
            let mut score = 0;
            let home_squares = match piece {
                Piece::Empty => unreachable!(),
                Piece::A => HOME_SQUARES_A,
                Piece::B => HOME_SQUARES_B,
                Piece::C => HOME_SQUARES_C,
                Piece::D => HOME_SQUARES_D,
            };
            for (i, &square) in home_squares.iter().enumerate() {
                let p = board.pieces[square];
                if p == piece {
                    score += 5 * (i + 1);
                } else if p == Piece::Empty {
                    score += 2 * (i + 1);
                }
            }
            score
        }

        let score = score_home(&self.board, Piece::A) * 10000
            + score_home(&self.board, Piece::B) * 1000
            + score_home(&self.board, Piece::C) * 100
            + score_home(&self.board, Piece::D) * 10;

        if score < self.cost {
            self.cost
        } else {
            usize::MAX - score
        }
    }
}

impl Board {
    // list of possible moves to make given the current board; sorted by cost (increasing)
    fn generate_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        for src in ALL_SQUARES {
            if self.pieces[src] == Piece::Empty {
                continue;
            }
            for (squares_to_pass, cost) in &POSSIBLE_MOVES[src] {
                if squares_to_pass
                    .iter()
                    .any(|&s| self.pieces[s] != Piece::Empty)
                {
                    continue;
                }
                let piece = self.pieces[src];
                let dest = squares_to_pass.last().unwrap();
                let home_squares = match piece {
                    Piece::Empty => unreachable!(),
                    Piece::A => HOME_SQUARES_A,
                    Piece::B => HOME_SQUARES_B,
                    Piece::C => HOME_SQUARES_C,
                    Piece::D => HOME_SQUARES_D,
                };
                if home_squares[3] == src {
                    continue;
                }
                if home_squares[2] == src && self.pieces[home_squares[3]] == piece {
                    continue;
                }
                if home_squares[1] == src
                    && self.pieces[home_squares[2]] == piece
                    && self.pieces[home_squares[3]] == piece
                {
                    continue;
                }
                if home_squares[0] == src
                    && self.pieces[home_squares[1]] == piece
                    && self.pieces[home_squares[2]] == piece
                    && self.pieces[home_squares[3]] == piece
                {
                    continue;
                }
                if X_SQUARES.contains(&src) {
                    if !home_squares.contains(dest) {
                        continue;
                    }
                    if home_squares
                        .iter()
                        .map(|&id| self.pieces[id])
                        .any(|p| p != Piece::Empty && p != piece)
                    {
                        continue;
                    }
                }
                let multiplier = match piece {
                    Piece::Empty => unreachable!(),
                    Piece::A => 1,
                    Piece::B => 10,
                    Piece::C => 100,
                    Piece::D => 1000,
                };
                let mut board = self.clone();
                board.pieces[src] = Piece::Empty;
                board.pieces[*dest] = piece;
                moves.push(Move {
                    board,
                    cost: cost * multiplier,
                });
            }
        }
        moves.sort_unstable_by_key(|mv| mv.score());
        moves
    }

    fn is_solved(&self) -> bool {
        *self == BOARD_SOLVED
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {} {} {} {}{} | {} {} {} {} | {} {} {} {} | {} {} {} {} | {} {} {} {}",
            self.pieces[X0],
            self.pieces[X1],
            self.pieces[X2],
            self.pieces[X3],
            self.pieces[X4],
            self.pieces[X5],
            self.pieces[X6],
            self.pieces[A3],
            self.pieces[B3],
            self.pieces[C3],
            self.pieces[D3],
            self.pieces[A2],
            self.pieces[B2],
            self.pieces[C2],
            self.pieces[D2],
            self.pieces[A1],
            self.pieces[B1],
            self.pieces[C1],
            self.pieces[D1],
            self.pieces[A0],
            self.pieces[B0],
            self.pieces[C0],
            self.pieces[D0],
        )
    }
}

impl TryFrom<&str> for Board {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = value.lines().collect();
        if lines.len() < 7
            || lines[1].len() < 12
            || lines[2].len() < 10
            || lines[3].len() < 10
            || lines[4].len() < 10
            || lines[5].len() < 10
        {
            return Err(Error::BadInput);
        }
        let l1: Vec<_> = lines[1].chars().collect();
        let l2: Vec<_> = lines[2].chars().collect();
        let l3: Vec<_> = lines[3].chars().collect();
        let l4: Vec<_> = lines[4].chars().collect();
        let l5: Vec<_> = lines[5].chars().collect();
        let board = Board {
            pieces: [
                l1[1].try_into()?,
                l1[2].try_into()?,
                l1[4].try_into()?,
                l1[6].try_into()?,
                l1[8].try_into()?,
                l1[10].try_into()?,
                l1[11].try_into()?,
                l2[3].try_into()?,
                l2[5].try_into()?,
                l2[7].try_into()?,
                l2[9].try_into()?,
                l3[3].try_into()?,
                l3[5].try_into()?,
                l3[7].try_into()?,
                l3[9].try_into()?,
                l4[3].try_into()?,
                l4[5].try_into()?,
                l4[7].try_into()?,
                l4[9].try_into()?,
                l5[3].try_into()?,
                l5[5].try_into()?,
                l5[7].try_into()?,
                l5[9].try_into()?,
            ],
        };
        if board.pieces.iter().filter(|&&p| p == Piece::Empty).count() != 7
            || board.pieces.iter().filter(|&&p| p == Piece::A).count() != 4
            || board.pieces.iter().filter(|&&p| p == Piece::B).count() != 4
            || board.pieces.iter().filter(|&&p| p == Piece::C).count() != 4
            || board.pieces.iter().filter(|&&p| p == Piece::D).count() != 4
        {
            return Err(Error::BadInput);
        }
        Ok(board)
    }
}

fn solve(board: &Board) -> Result<usize, Error> {
    fn visit(board: &Board, cost: usize, paths: &mut FxHashMap<Board, usize>) {
        if board.is_solved() {
            return;
        }
        if let Some(&best_cost) = paths.get(&BOARD_SOLVED) {
            if cost > best_cost {
                return;
            }
        }
        if let Some(&earlier_cost) = paths.get(board) {
            if cost > earlier_cost {
                return;
            }
        }
        for mv in board.generate_moves() {
            let tentative_cost = cost + mv.cost;
            if let Some(&earlier_cost) = paths.get(&mv.board) {
                if tentative_cost >= earlier_cost {
                    continue;
                }
            }
            paths.insert(mv.board.clone(), tentative_cost);
            visit(&mv.board, tentative_cost, paths);
        }
    }
    let mut paths = FxHashMap::default();
    paths.insert(board.clone(), 0);
    visit(board, 0, &mut paths);
    if let Some(&cost) = paths.get(&BOARD_SOLVED) {
        return Ok(cost);
    }
    Err(Error::NoSolution)
}

fn part_one(input: &str) -> Result<usize, Error> {
    solve(&input.try_into()?)
}

fn part_two(input: &str) -> Result<usize, Error> {
    solve(&input.try_into()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = include_str!("test-input-part-one.txt");

    #[test]
    fn test_board_try_from() {
        let _: Board = INPUT1.try_into().unwrap();
    }

    #[test]
    fn test_board_is_solved() {
        let board: Board = INPUT1.try_into().unwrap();
        assert!(!board.is_solved());

        let board: Board = r#"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########"#
            .try_into()
            .unwrap();
        assert!(board.is_solved());
    }

    #[test]
    fn test_board_generate_moves() {
        let board: Board = INPUT1.try_into().unwrap();
        let moves = board.generate_moves();
        assert_eq!(moves.len(), 4 * 7);
    }

    #[test]
    #[ignore]
    fn test_part_one() {
        assert_eq!(part_one(INPUT1), Ok(12521));
    }
}
