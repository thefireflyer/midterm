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

pub fn hanoi_general_rec(m: usize, d: u32, pegs: &mut Vec<Vec<u32>>) {
    assert!(pegs.len() >= 3);

    if m == 0 {
        return;
    } else if m == 1 {
        // todo: origin and target need to be variable
        let val = pegs[0].pop().unwrap();
        pegs.last_mut().unwrap().push(val);
    } else if m == 2 {
        let val = pegs[0].pop().unwrap();
        pegs[1].push(val);

        let val = pegs[0].pop().unwrap();
        pegs.last_mut().unwrap().push(val);

        let val = pegs[1].pop().unwrap();
        pegs.last_mut().unwrap().push(val);
    } else {
        let val = pegs[0].last().unwrap().clone();

        let (source, others) = pegs.split_at_mut(1);

        // spread out
        for peg in others
            .iter_mut()
            .filter(|peg| peg.last().is_some_and(|x| x < &val))
        {
            let val = source[0].pop().unwrap();
            peg.push(val);
        }
    }
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
}

///////////////////////////////////////////////////////////////////////////////
