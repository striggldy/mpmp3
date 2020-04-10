use std::clone::Clone;
use std::iter::Iterator;
/*
0 points - blank (x2)

1 point - A (x9), E (x12), I (x9), O (x8), U (x4), L (x4), N (x6), S (x4), T (x6), R (x6)

2 points -D (x4), G (x3)

3 points -B (x2), C (x2), M (x2), P (x2)

4 points -F (x2), H (x2), V (x2), W (x2), Y (x2)

5 points -K (x1)

8 points - J (x1), X (x1)

10 points -Q (x1), Z (x1)
*/
#[derive(Debug)]
pub struct Piece {
    pub chr: char,
    pub val: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Pieces {
    pub chr: char,
    pub val: u8,
    pub num: u8,
}

enum RecError {
    TooBig,
    TooSmall,
    NoSolution,
    Done,
}

struct Solver {
    solution: Vec<String>,
    n_pieces: u8,
    points: u8,
    collection: Vec<Pieces>,
}

impl Solver {
    pub fn new() -> Self {
        let collection = Solver::setup();

        Solver {
            solution: Vec::new(),
            n_pieces: 0,
            points: 0,
            collection,
        }
    }

    fn find_one(
        &mut self,
        piece_idx: &mut usize,
        points_left: &mut u8,
        sol: &mut String,
        coll: &mut [Pieces],
    ) -> Result<(), RecError> {
        let pieces_left = self.n_pieces - sol.len() as u8;
        //let mut current_piece = coll.get_mut(*piece_idx).unwrap();

        println!(
            "Starting: {:?}, {}, {}, {}",
            current_piece, points_left, piece_idx, sol
        );

        if current_piece.val > *points_left {
            // no room for this piece
            println!("TooBig: {:?}, {}, {}", current_piece, points_left, sol);
            return Err(RecError::TooBig);
        }

        if *points_left < pieces_left * current_piece.val {
            // if we insert our piece, no other pieces will fit anymore, because the piece values are in icrementing order
            println!("TooSmall: {:?}, {}, {}", current_piece, points_left, sol);
            return Err(RecError::TooSmall);
        }

        if pieces_left == 1 && (*points_left - current_piece.val > 0) {
            println!("NoSolution: {:?}, {}, {}", current_piece, points_left, sol);
            return Err(RecError::NoSolution);
        }

        *points_left -= current_piece.val;
        current_piece.num -= 1;
        // println!(
        //     "After change: current: {}, coll: {}",
        //     current_piece.num, coll[0].num
        // );
        sol.push_str(&current_piece.chr.to_string());

        if *points_left == 0 && pieces_left == 1 {
            println!("Ok: {:?}, {}, {}", current_piece, points_left, sol);
            return Ok(());
        } else {
            loop {
                if current_piece.num == 0 {
                    *piece_idx += 1;
                    if *piece_idx > self.n_pieces as usize {
                        return Err(RecError::Done);
                    }
                }

                println!(
                    "Piece idx: {}, Piece.num: {}",
                    *piece_idx, current_piece.num
                );

                match self.find_one(piece_idx, points_left, sol, coll) {
                    Ok(sol) => return Ok(sol),
                    Err(RecError::TooBig) => return Err(RecError::TooBig),
                    Err(RecError::TooSmall) => {}
                    Err(RecError::NoSolution) => {
                        if current_piece.num == 0 {
                            *piece_idx += 1;
                            if *piece_idx > self.n_pieces as usize {
                                return Err(RecError::Done);
                            }
                        } else {
                            current_piece.num -= 1;
                        }
                        println!("NoSultion iter: {:?}", current_piece);
                    }
                    Err(RecError::Done) => {
                        println!("Done within inner loop?");
                        return Err(RecError::Done);
                    }
                }
            }
        }
    }

    fn solve(&mut self, n_pieces: u8, points: u8) -> &Vec<String> {
        self.n_pieces = n_pieces;
        self.points = points;

        let mut this_sol = String::new();
        let mut coll = self.collection.clone();

        match self.find_one(&mut 0, &mut points.clone(), &mut this_sol, &mut coll) {
            Ok(_) => self.solution.push(this_sol),
            _ => panic!("HELP!"),
        }

        &self.solution
    }

    fn setup() -> Vec<Pieces> {
        let desc = [
            // char, value, amount
            ('0', 0, 2),
            ('A', 1, 9),
            ('E', 1, 12),
            ('I', 1, 9),
            ('O', 1, 8),
            ('U', 1, 4),
            ('L', 1, 4),
            ('N', 1, 6),
            ('S', 1, 4),
            ('T', 1, 6),
            ('R', 1, 6),
            ('D', 2, 4),
            ('G', 2, 3),
            ('B', 3, 2),
            ('C', 3, 2),
            ('M', 3, 2),
            ('P', 3, 2),
            ('F', 4, 2),
            ('H', 4, 2),
            ('V', 4, 2),
            ('W', 4, 2),
            ('Y', 4, 2),
            ('K', 5, 1),
            ('J', 8, 1),
            ('X', 8, 1),
            ('Q', 10, 1),
            ('Z', 10, 1),
        ];
        // let full_collection: Vec<_> = desc
        //     .iter()
        //     .flat_map(|(chr, val, num)| {
        //         (0..*num).map(move |_| Piece {
        //             chr: *chr,
        //             val: *val,
        //         })
        //     })
        //     .collect();
        let collection: Vec<_> = desc
            .iter()
            .map(|(chr, val, num)| Pieces {
                chr: *chr,
                val: *val,
                num: *num,
            })
            .collect();
        collection
    }
}

fn main() {
    let mut solver = Solver::new();
    let solution = solver.solve(7, 46);

    println!("{:?}", solution);
}
