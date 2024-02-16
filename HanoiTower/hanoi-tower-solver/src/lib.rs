///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////

/*

o a t
1
2
3
_ _ _

2
3   1
_ _ _

3 2 1
_ _ _

  1
3 2
_ _ _

  1
  2 3
_ _ _

1 2 3
_ _ _

    2
1   3
_ _ _

    1
    2
    3
_ _ _

*/

///////////////////////////////////////////////////////////////////////////////

pub enum pole {
    o,
    a,
    t,
}

pub fn hanoi_simple_rec<'a>(
    o: &'a [u32],
    a: &'a [u32],
    mut t: &'a [u32],
    ms: &mut Vec<(pole, pole)>,
) -> (&'a [u32], &'a [u32], &'a [u32]) {
    if o.is_empty() {
        return (o, a, t);
    }

    let size = o.len();
    let (o2, t2, a2) = hanoi_simple_rec(&o[..size], t, a, ms);

    ms.push((pole::o, pole::t));

    t = &[t, &o[size..]].concat();
    let o = &o[size + 1..];

    hanoi_simple_rec(a, o, &t, ms);

    (o, a, &t)
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanoi_simple_rec() {
        hanoi_simple_rec(&mut state);
        assert_eq!(
            state,
            State {
                origin: vec![],
                target: vec![5, 4, 3, 2, 1],
                aux: vec![],
            }
        )
    }
}

///////////////////////////////////////////////////////////////////////////////
