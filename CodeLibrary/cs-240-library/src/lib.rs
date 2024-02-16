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
