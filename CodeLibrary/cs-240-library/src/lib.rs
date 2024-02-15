/*

--- Progress

- [x] copy over algorithms
    - [x] linear search
    - [x] binary search
    - [x] selection sort
    - [x] insertion sort
    - [x] quick sort
    - [x] merge sort

- [ ] linked lists
    - [x] copy over single linked list
    - [ ] double linked list
        - [ ] unit testing

- [ ] stacks
    - [ ] with array
    - [ ] with linked list
    - [ ] unit testing

- [ ] queues
    - [ ] with array
    - [ ] with linked list
    - [ ] unit testing

- [ ] clean up and review
    - [ ] clean up algorithms
    - [ ] clean up single linked list


--- Time allocation

    [MAX] end of the 15th
    [IDEAL] end of today

*/

///////////////////////////////////////////////////////////////////////////////

pub mod algorithms {

    pub mod search {
        pub mod binary_search;
        pub mod linear_search;
    }

    //.......................................................................//

    pub mod sort {
        pub mod selection_sort {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        pub mod insertion_sort {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        pub mod merge_sort;
        pub mod quick_sort;

        #[cfg(test)]
        mod shared_test_cases;
    }
}

//---------------------------------------------------------------------------//

pub mod data_structures {

    pub mod linked_list {
        pub mod single_linked_list {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        pub mod double_linked_list {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        #[cfg(test)]
        mod shared_test_cases;
    }

    //.......................................................................//

    pub mod stack {

        pub mod linked_stack {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        pub mod array_stack {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        #[cfg(test)]
        mod shared_test_cases;
    }

    //.......................................................................//

    pub mod queue {

        pub mod linked_queue {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        pub mod array_queue {
            pub mod solution;

            #[cfg(test)]
            mod tests;
        }

        #[cfg(test)]
        mod shared_test_cases;
    }
}

///////////////////////////////////////////////////////////////////////////////

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
