///////////////////////////////////////////////////////////////////////////////

use hanoi_tower_solver::{debug, hanoi_simple_iter, hanoi_simple_rec};

///////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut o = vec![4, 3, 2, 1];
    let mut a = vec![];
    let mut t = vec![];
    hanoi_simple_rec(4, &mut o, &mut a, &mut t, 0, &|o, a, t, d| {
        debug(o, a, t, d)
    });

    println!();

    let mut o = vec![4, 3, 2, 1];
    let mut a = vec![];
    let mut t = vec![];
    hanoi_simple_iter(&mut o, &mut a, &mut t);
}

///////////////////////////////////////////////////////////////////////////////
