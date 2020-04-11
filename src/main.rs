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
    RemoveAndSkipToNextPiece,
    SkipMeToNextPiece,
    SkipMeToNextBlock,
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

    fn find_all(
        &mut self,
        piece_idx: usize,
        points_left: u8,
        sol: &mut String,
        coll: &mut [Pieces],
    ) -> Result<(), RecError> {
        let pieces_left = self.n_pieces - sol.len() as u8;

        if coll[piece_idx].val > points_left {
            // no room for this piece -> parent is too big as well
            return Err(RecError::RemoveAndSkipToNextPiece);
        }

        if points_left < pieces_left * coll[piece_idx].val {
            // if we insert our piece, no other pieces will fit anymore, because the piece values are in icrementing order
            // -> this piece is too small
            return Err(RecError::TooSmall);
        }

        let points_left_after = points_left - coll[piece_idx].val;
        if points_left_after > Solver::max_size_of_last_n_elements(pieces_left - 1) {
            // there's no possible way to reach the points on this branch
            return Err(RecError::SkipMeToNextBlock);
        }
        coll[piece_idx].num -= 1;
        sol.push_str(&coll[piece_idx].chr.to_string());

        if points_left_after == 0 && pieces_left == 1 {
            // we found one!
            self.solution.push((*sol).clone());
            return Ok(());
        } else {
            let mut next_search_idx = piece_idx;

            if coll[piece_idx].num == 0 {
                // no more pieces of this char left -> move to the next piece
                next_search_idx += 1;
            }

            loop {
                if next_search_idx >= coll.len() {
                    *sol = sol[..sol.len() - 1].to_string();
                    return Err(RecError::SkipMeToNextPiece);
                }
                match self.find_all(next_search_idx, points_left_after, sol, coll) {
                    Ok(_) => {
                        *sol = sol[..sol.len() - 1].to_string();
                        next_search_idx += 1;
                    }
                    Err(RecError::TooSmall) => {
                        next_search_idx =
                            Solver::get_index_of_next_point_bracket(coll[next_search_idx].val);
                    }
                    Err(RecError::SkipMeToNextBlock) => {
                        for i in piece_idx..coll.len() {
                            coll[i].num = self.collection[i].num;
                        }
                        next_search_idx =
                            Solver::get_index_of_next_point_bracket(coll[next_search_idx].val);
                    }
                    Err(RecError::SkipMeToNextPiece) => {
                        for i in piece_idx..coll.len() {
                            coll[i].num = self.collection[i].num;
                        }
                        next_search_idx += 1;
                    }
                    Err(RecError::RemoveAndSkipToNextPiece) => {
                        *sol = sol[..sol.len() - 1].to_string();
                        return Err(RecError::SkipMeToNextPiece);
                    }
                }
            }
        }
    }

    fn solve(&mut self, n_pieces: u8, points: u8) -> &Vec<String> {
        self.n_pieces = n_pieces;
        self.points = points;

        self.solution.clear();

        let mut coll = self.collection.clone();
        for idx in 0..self.collection.len() {
            match self.find_all(idx, points, &mut String::new(), &mut coll) {
                _ => {}
            }
        }
        &self.solution
    }

    fn get_index_of_next_point_bracket(current_val: u8) -> usize {
        match current_val {
            0 => 1,
            1 => 11,
            2 => 13,
            3 => 17,
            4 => 22,
            5 => 23,
            8 => 25,
            10 => 27,
            _ => unreachable!("next_idx for val>10"),
        }
    }

    fn max_size_of_last_n_elements(free_elements: u8) -> u8 {
        match free_elements {
            0 => 0,
            1 => 10,
            2 => 20,
            3 => 28,
            4 => 36,
            5 => 41,
            6 => 45,
            _ => unreachable!("free elements"),
        }
    }

    fn setup() -> Vec<Pieces> {
        let desc = [
            // char, value, number
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
    let now = std::time::Instant::now();
    let mut solver = Solver::new();

    for i in 0..=50 {
        let solution = solver.solve(7, i);
        println!("{}: {}", i, solution.len())
    }
    println!("Took: {}s", now.elapsed().as_secs());

    // let solution = solver.solve(7, 46);
    // let s = solution.join("\n");
    // println!("{}", s);
    // println!("{}", solution.len());
}
