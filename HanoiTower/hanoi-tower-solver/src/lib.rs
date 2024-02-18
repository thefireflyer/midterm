///////////////////////////////////////////////////////////////////////////////

pub fn debug(o: &mut Vec<u32>, a: &mut Vec<u32>, t: &mut Vec<u32>, d: u32) {
    print!("{} | ", d);
    for _ in 0..d {
        print!("|  ");
    }
    println!("o: {:?} a: {:?} t: {:?}", o, a, t);
    check(o, a, t, d);
}

//---------------------------------------------------------------------------//

pub fn check(o: &mut Vec<u32>, a: &mut Vec<u32>, t: &mut Vec<u32>, d: u32) {
    if !o
        .iter()
        .fold((true, u32::MAX), |(res, acc), e| (res && e < &acc, *e))
        .0
    {
        println!("unsorted org");
        panic!("unsorted org")
    }
    if !a
        .iter()
        .fold((true, u32::MAX), |(res, acc), e| (res && e < &acc, *e))
        .0
    {
        println!("unsorted aux");
        panic!("unsorted aux")
    }
    if !t
        .iter()
        .fold((true, u32::MAX), |(res, acc), e| (res && e < &acc, *e))
        .0
    {
        println!("unsorted target");
        panic!("unsorted target")
    }
}

///////////////////////////////////////////////////////////////////////////////

pub fn hanoi_simple_rec(
    m: usize,
    o: &mut Vec<u32>,
    a: &mut Vec<u32>,
    t: &mut Vec<u32>,
    d: u32,
    de: &dyn Fn(&mut Vec<u32>, &mut Vec<u32>, &mut Vec<u32>, u32),
) {
    de(o, a, t, d);

    if m == 0 {
        return;
    } else if m == 1 {
        t.push(o.pop().unwrap());
        de(o, a, t, d);
    } else if m == 2 {
        a.push(o.pop().unwrap());
        de(o, a, t, d);
        t.push(o.pop().unwrap());
        de(o, a, t, d);
        t.push(a.pop().unwrap());
        de(o, a, t, d);
    } else {
        let last = o.remove(o.len() - m);
        hanoi_simple_rec(m - 1, o, t, a, d + 1, &|o, a, t, d| {
            de(o, t, a, d);
        });
        t.push(last);
        de(o, a, t, d);

        hanoi_simple_rec(m - 1, a, o, t, d + 1, &|o, a, t, d| {
            de(a, o, t, d);
        });

        de(o, a, t, d);
    }
}

//---------------------------------------------------------------------------//

pub fn hanoi_simple_iter(o: &mut Vec<u32>, a: &mut Vec<u32>, t: &mut Vec<u32>) {
    fn inner(a: &mut Vec<u32>, b: &mut Vec<u32>) {
        let vala = a.last();
        let valb = b.last();

        match (vala, valb) {
            (None, None) => {}
            (None, Some(_)) => a.push(b.pop().unwrap()),
            (Some(_), None) => b.push(a.pop().unwrap()),
            (Some(vala), Some(valb)) => {
                if vala < valb {
                    b.push(a.pop().unwrap());
                } else {
                    a.push(b.pop().unwrap());
                }
            }
        }
    }

    while (!o.is_empty()) || (!a.is_empty()) {
        println!("o: {:?} a: {:?} t: {:?}", o, a, t);

        inner(o, a);
        println!("o: {:?} a: {:?} t: {:?}", o, a, t);

        inner(o, t);
        println!("o: {:?} a: {:?} t: {:?}", o, a, t);

        inner(a, t);
        println!("o: {:?} a: {:?} t: {:?}", o, a, t);
    }
}

///////////////////////////////////////////////////////////////////////////////

/*
--- Variables pegs and disks

Pegs = {} of k length
Disks = [] of n length

If k >= n, then just lay out the disk across n pegs and then collect them for a total of 2n-1 actions

Otherwise,



*/

///////////////////////////////////////////////////////////////////////////////

fn debug2(m: usize, d: u32, pegs: &Vec<Vec<u32>>, source: usize, target: usize) {
    print!("{} | ", d);
    for _ in 0..d {
        print!("|  ");
    }
    for peg in pegs {
        print!("{:?}", peg);
    }
    print!(
        "   ( s: {}{:?} t: {}{:?} )",
        source, pegs[source], target, pegs[target]
    );
    println!();

    for peg in pegs {
        if !peg
            .iter()
            .fold((true, u32::MAX), |(res, acc), e| (res && e < &acc, *e))
            .0
        {
            println!("unsorted peg");
            panic!("unsorted peg")
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

pub fn hanoi_general_rec(
    m: usize,
    d: u32,
    pegs: &mut Vec<Vec<u32>>,
    source: usize,
    target: usize,
    aux: Vec<usize>,
    moves: &mut Vec<(usize, usize, u32)>,
) {
    assert!(pegs.len() >= 3);
    // assert!(d < 15);

    debug2(m, d, pegs, source, target);

    if m == 0 {
        return;
    } else if m == 1 {
        let val = pegs[source].pop().unwrap();
        pegs[target].push(val);
        debug2(m, d, pegs, source, target);
        moves.push((source, target, val));
    } else if m == 2 {
        let val = pegs[source].pop().unwrap();
        pegs[aux[0]].push(val);
        debug2(m, d, pegs, source, target);
        moves.push((source, aux[0], val));

        let val = pegs[source].pop().unwrap();
        pegs[target].push(val);
        debug2(m, d, pegs, source, target);
        moves.push((source, target, val));

        let val = pegs[aux[0]].pop().unwrap();
        pegs[target].push(val);
        debug2(m, d, pegs, source, target);
        moves.push((aux[0], target, val));
    } else {
        let k = aux.len();

        if m <= k + 1 {
            println!("extra pegs");
            for i in 0..m - 1 {
                let val = pegs[source].pop().unwrap();
                pegs[aux[i]].push(val);
                debug2(m, d, pegs, source, target);
                moves.push((source, aux[i], val));
            }
            let val = pegs[source].pop().unwrap();
            pegs[target].push(val);
            for i in (0..m - 1).rev() {
                let val = pegs[aux[i]].pop().unwrap();
                pegs[target].push(val);
                debug2(m, d, pegs, source, target);
                moves.push((aux[i], target, val));
            }
        } else {
            let mut vals = vec![];

            let bottom = pegs[source].len() - m;
            let val = pegs[source].remove(bottom);

            for i in 1..k {
                if pegs[source].is_empty() {
                    break;
                } else {
                    let val = pegs[source].remove(bottom);
                    vals.push(val);
                }
            }

            hanoi_general_rec(m - k, d + 1, pegs, source, aux[0], vec![target], moves);
            debug2(m, d, pegs, source, target);

            for i in 1..k {
                if vals.is_empty() {
                    break;
                } else {
                    let val = vals.pop().unwrap();
                    pegs[aux[i]].push(val);
                    debug2(m, d, pegs, source, target);
                    moves.push((source, aux[i], val));
                }
            }

            pegs[target].push(val);
            debug2(m, d, pegs, source, target);
            moves.push((source, target, val));

            for i in (1..k).rev() {
                if pegs[aux[i]].is_empty() {
                    break;
                } else {
                    let val = pegs[aux[i]].pop().unwrap();
                    pegs[target].push(val);
                    debug2(m, d, pegs, source, target);
                    moves.push((aux[i], target, val));
                }
            }

            hanoi_general_rec(m - k, d + 1, pegs, aux[0], target, vec![source], moves);
        }
    }

    debug2(m, d, pegs, source, target);
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanoi_simple_rec() {
        for i in 0..20 {
            let f: Vec<u32> = (0..i).rev().collect();
            let mut o = f.clone();
            let mut a = vec![];
            let mut t = vec![];
            hanoi_simple_rec(
                i.try_into().unwrap(),
                &mut o,
                &mut a,
                &mut t,
                0,
                &|o, a, t, d| debug(o, a, t, d),
            );
            assert_eq!(o, vec![]);
            assert_eq!(a, vec![]);
            assert_eq!(t, f.clone());
            println!();
        }
    }

    #[test]
    fn test_hanoi_simple_iter() {
        for i in 0..20 {
            let f: Vec<u32> = (0..i).rev().collect();
            let mut o = f.clone();
            let mut a = vec![];
            let mut t = vec![];
            hanoi_simple_iter(&mut o, &mut a, &mut t);
            assert_eq!(o, vec![]);
            assert_eq!(a, vec![]);
            assert_eq!(t, f.clone());
            println!();
        }
    }

    #[test]
    fn test_hanoi_rec() {
        for i in 3..20 {
            for j in 0..20 {
                println!("{} pegs with {} disks", i, j);
                let f: Vec<u32> = (0..j).rev().collect();
                let mut rods = vec![];
                for n in 0..i {
                    rods.push(vec![]);
                }
                rods[0] = f.clone();
                hanoi_general_rec(
                    j.try_into().unwrap(),
                    0,
                    &mut rods,
                    0,
                    i - 1,
                    (1..i - 1).collect(),
                    &mut vec![],
                );
                for n in 0..i - 1 {
                    assert_eq!(rods[n], vec![]);
                }
                assert_eq!(rods[i - 1], f.clone());
                println!();
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
