use std::iter::Iterator;
use std::ops::Range;
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

#[derive(Debug)]
pub struct Pieces {
    pub chr: char,
    pub val: u8,
    pub num: u8,
}

fn check(col: Vec<Pieces>) -> Vec<Vec<Piece>> {
    let mut result = Vec::new();
    for p in col {
        let r: Vec<_> = (0..3)
            .into_iter()
            .map(|_| Piece { chr: 'A', val: 3 })
            .collect();
        result.push(r);
    }
    result
}

fn main() {
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
    let full_collection: Vec<_> = desc
        .iter()
        .flat_map(|(chr, val, num)| {
            (0..*num).map(move |_| Piece {
                chr: *chr,
                val: *val,
            })
        })
        .collect();

    let collection: Vec<_> = desc
        .iter()
        .map(|(chr, val, num)| Pieces {
            chr: *chr,
            val: *val,
            num: *num,
        })
        .collect();
    println!("{:?}", full_collection.len());
    println!("{:?}", collection.len());

    let r = check(collection);
    println!("{:?}", r);
}
