# Code library 

| | |
|-|-|
| Author | Aidan Beil |
| Date | 14/2/2024 |
| Class | CS240 2963 |
| Professor | Darrell Criss |

## Organization

- [Hash Set](cs-240-library/src/data_structures/hashset.rs)
- Hash Tables
    - [Hashing algorithm](combined/Hasher.cs)
    - Using open addressing - [`OpenHashMap`](combined/Tables/OpenHashTable.cs)
    - Using closed addressing - [`ClosedHashMap`](combined/Tables/ClosedHashTable.cs)
    - [Unit testing](combined/Tables/TestTable.cs)
- Maps
    - [Interface](combined/Maps/IMap.cs)
    - BST Maps
        - Unbalanced - [`BST`](combined/Maps/BSTs/BST.cs)
    - [Unit testing](combined/Maps/TestMaps.cs)
- Stacks
    - [Interface](combined/Stacks/IStack.cs)
    - Using linked lists - [`LinkedStack`](combined/Stacks/LinkedStack.cs)
    - Using arrays - [`ArrayStack`](combined/Stacks/ArrayStack.cs)
    - [Unit testing](combined/Stacks/TestStacks.cs)
- Queues
    - [Interface](combined/Queues/IQueue.cs)
    - Using linked lists - [`LinkedQueue`](combined/Queues/LinkedQueue.cs)
    - Using arrays - [`ArrayQueue`](combined/Queues/ArrayQueue.cs)
    - [Unit testing](combined/Queues/TestQueues.cs)
- Linked Lists
    - Double linked list
        - [`LinkedList`](combined/LinkedList/LinkedList.cs)
        - [Unit testing](combined/LinkedList/Test.cs)
    - Single linked list
        - [`LinkedList`](cs-240-library/src/data_structures/linked_list/single_linked_list/solution.rs)
        - [Unit testing](cs-240-library/src/data_structures/linked_list/single_linked_list/tests.rs)
- Search algorithms
    - [Binary search](cs-240-library/src/algorithms/search/binary_search.rs)
    - [Linear search](cs-240-library/src/algorithms/search/linear_search.rs)
- Sorting algorithms
    - [Insertion sort](cs-240-library/src/algorithms/sort/insertion_sort/solution.rs)
        - [Unit testing](cs-240-library/src/algorithms/sort/insertion_sort/tests.rs)
    - [Selection sort](cs-240-library/src/algorithms/sort/selection_sort/solution.rs)
        - [Unit testing](cs-240-library/src/algorithms/sort/selection_sort/tests.rs)
    - [Merge sort](cs-240-library/src/algorithms/sort/merge_sort.rs)
    - [Quick sort](cs-240-library/src/algorithms/sort/quick_sort.rs)

## Usage

```
cd cs-240-library
cargo test
```

> ```
> running 41 tests
> test algorithms::search::binary_search::tests::test_existent_target ... ok
> test algorithms::search::binary_search::tests::test_empty_list ... ok
> test algorithms::search::linear_search::tests::test_empty_list ... ok
> test algorithms::search::binary_search::tests::test_nonexistent_target ... ok
> test algorithms::search::linear_search::tests::test_existent_target ... ok
> test algorithms::search::linear_search::tests::test_nonexistent_target ... ok
> test algorithms::sort::insertion_sort::tests::random_cases ... ok
> test algorithms::sort::insertion_sort::tests::reverse_sorted_cases ... ok
> test algorithms::sort::insertion_sort::tests::sorted_cases ... ok
> test algorithms::sort::insertion_sort::tests::special_cases ... ok
> test algorithms::sort::merge_sort::tests::random_cases ... ok
> test algorithms::sort::merge_sort::tests::reverse_sorted_cases ... ok
> test algorithms::sort::merge_sort::tests::special_cases ... ok
> test algorithms::sort::merge_sort::tests::sorted_cases ... ok
> test algorithms::sort::merge_sort::tests::test_big_rev_sorted ... ok
> test algorithms::sort::selection_sort::tests::random_cases ... ok
> test algorithms::sort::quick_sort::tests::reverse_sorted_cases ... ok
> test algorithms::sort::quick_sort::tests::sorted_cases ... ok
> test algorithms::sort::quick_sort::tests::special_cases ... ok
> test algorithms::sort::quick_sort::tests::random_cases ... ok
> test algorithms::sort::selection_sort::tests::reverse_sorted_cases ... ok
> test data_structures::hashset::tests::test_basics ... ok
> test data_structures::linked_list::double_linked_list::tests::test_basic ... ok
> test algorithms::sort::selection_sort::tests::sorted_cases ... ok
> test data_structures::linked_list::single_linked_list::tests::base ... ok
> test data_structures::linked_list::single_linked_list::tests::delete ... ok
> test data_structures::linked_list::single_linked_list::tests::insert ... ok
> test data_structures::linked_list::single_linked_list::tests::into_iter ... ok
> test data_structures::linked_list::single_linked_list::tests::iter ... ok
> test data_structures::linked_list::double_linked_list::tests::test_basic_front ... ok
> test data_structures::linked_list::single_linked_list::tests::iter_mut ... ok
> test algorithms::sort::selection_sort::tests::special_cases ... ok
> test algorithms::sort::quick_sort::tests::test_big_rev_sorted ... ok
> test data_structures::linked_list::single_linked_list::tests::search ... ok
> test tests::it_works ... ok
> test data_structures::linked_list::single_linked_list::tests::sort ... ok
> test data_structures::linked_list::single_linked_list::tests::miri_testing_from_book_ref ... ok
> test data_structures::linked_list::single_linked_list::tests::peek ... ok
> test data_structures::linked_list::single_linked_list::tests::read ... ok
> test algorithms::sort::quick_sort::tests::test_big_sorted ... ok
> test algorithms::sort::merge_sort::tests::test_big_sorted ... ok
> 
> test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.22s
> 
>    Doc-tests cs-240-library
> 
> running 0 tests
> 
> test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
> ```

```
cd combined
dotnet test
```

> ```
> Starting test execution, please wait...
> A total of 1 test files matched the specified pattern.
> 
> Passed!  - Failed:     0, Passed:    13, Skipped:     0, Total:    13, Duration: 38 ms - combined.dll (net8.0)
> ```

## Approach

I really wanted to rewrite everything in Rust, but I ran out of time. The one thing I did get ported was the hash set (which is an actual set, not a table). The process was pretty simple since everything was easy to write in safe code.

## Review

I would like to come back to this in the future and finishing porting everything to Rust, but I am still fairly happy with what I have currently.