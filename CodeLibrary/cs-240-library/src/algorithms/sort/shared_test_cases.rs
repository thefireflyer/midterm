///////////////////////////////////////////////////////////////////////////////

pub fn shared_special_cases() -> Vec<Vec<i32>> {
    vec![vec![], vec![1]]
}

//---------------------------------------------------------------------------//

pub fn shared_random_cases() -> Vec<Vec<i32>> {
    vec![
        vec![1, 3, 2],
        vec![1, 3, 2],
        vec![3, 3, 3],
        vec![3, 3, 2],
        vec![593, 52, 0, 40104, 20, 19, 2, 30, 8],
        vec![5, 23, 6, 8, 9, 0, 2],
    ]
}

//---------------------------------------------------------------------------//

pub fn shared_sorted_cases() -> Vec<Vec<i32>> {
    vec![
        vec![1, 2],
        vec![1, 2, 3],
        vec![0, 2, 5, 6, 8, 9, 23],
        vec![-503, 1, 203, 585, 900],
    ]
}

//---------------------------------------------------------------------------//

pub fn shared_reverse_sorted_cases() -> Vec<Vec<i32>> {
    vec![
        vec![2, 1],
        vec![3, 2, 1],
        vec![5, 4, 3, 2, 1, 0, -40],
        vec![23, 9, 8, 6, 5, 2, 0],
    ]
}

///////////////////////////////////////////////////////////////////////////////
