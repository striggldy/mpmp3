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
#[derive(Debug, Clone, Copy)]
pub struct Pieces {
    pub chr: char,
    pub val: u8,
    pub num: u8,
}

enum RecError {
    TooSmall,
    NoSolution,
    SkipMe,
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
        piece_idx: usize,
        points_left: u8,
        sol: &mut String,
        coll: &mut [Pieces],
    ) -> Result<(), RecError> {
        let pieces_left = self.n_pieces - sol.len() as u8;

        if coll[piece_idx].val > points_left {
            // no room for this piece
            return Err(RecError::NoSolution);
        }

        if points_left < pieces_left * coll[piece_idx].val {
            // if we insert our piece, no other pieces will fit anymore, because the piece values are in icrementing order
            return Err(RecError::TooSmall);
        }

        let points_left_after = points_left - coll[piece_idx].val;

        if pieces_left == 1 && (points_left_after > 0) {
            return Err(RecError::TooSmall);
        }

        coll[piece_idx].num -= 1;
        sol.push_str(&coll[piece_idx].chr.to_string());

        if points_left_after == 0 && pieces_left == 1 {
            self.solution.push((*sol).clone());
            return Ok(());
        } else {
            let mut next_search_idx = piece_idx;

            if coll[piece_idx].num == 0 {
                next_search_idx += 1;
            }

            loop {
                if next_search_idx >= coll.len() {
                    *sol = sol[..sol.len() - 1].to_string();
                    // if piece_idx == 0 {
                    //     return Ok(());
                    // }
                    return Err(RecError::SkipMe);
                }
                match self.find_one(next_search_idx, points_left_after, sol, coll) {
                    Ok(_) => {
                        *sol = sol[..sol.len() - 1].to_string();
                        next_search_idx += 1;
                    }
                    Err(RecError::TooSmall) => {
                        next_search_idx += 1;
                    }
                    Err(RecError::SkipMe) => {
                        for i in piece_idx..coll.len() {
                            coll[i].num = self.collection[i].num;
                        }
                        next_search_idx += 1;
                    }
                    Err(RecError::NoSolution) => {
                        // we reached Z and there's no solution
                        // remove ourself and allow iterating again

                        *sol = sol[..sol.len() - 1].to_string();
                        return Err(RecError::SkipMe);
                    }
                }
            }
        }
    }

    fn solve(&mut self, n_pieces: u8, points: u8) -> &Vec<String> {
        self.n_pieces = n_pieces;
        self.points = points;

        let mut this_sol = String::new();

        for idx in 0..self.collection.len() {
            let mut coll = self.collection.clone();
            match self.find_one(idx, points, &mut this_sol, &mut coll) {
                _ => println!("{} done", coll[idx].chr),
            }
        }

        println!("{:?}", self.solution);
        println!("{}", self.solution.len());

        self.solution.sort();
        self.solution.dedup();
        println!("{}", self.solution.len());

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
    solver.solve(7, 46);
}
