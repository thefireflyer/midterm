# Hanoi's Tower

| | |
|-|-|
| Author | Aidan Beil |
| Date | 14/2/2024 |
| Class | CS240 2963 |
| Professor | Darrell Criss |

![Hanoi tower video]()

## Organization

- [solver](hanoi-tower-solver/src/lib.rs)
- gui
    - [main](hanoi-tower-gui/src/main.rs)
    - [components](hanoi-tower-gui/src/components.rs)
    - [events](hanoi-tower-gui/src/events.rs)
    - [resources](hanoi-tower-gui/src/resources.rs)
    - [system](hanoi-tower-gui/src/systems.rs) (most of the interesting stuff)

## Usage

```
cd hanoi-tower-solver
cargo test
```

> ```
> running 3 tests
> test tests::test_hanoi_simple_iter ... ok
> test tests::test_hanoi_simple_rec ... ok
> test tests::test_hanoi_rec ... ok
> 
> test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.64s
> 
>    Doc-tests hanoi-tower-solver
> 
> running 0 tests
> 
> test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
> ```

```
cd hanoi-tower-gui
cargo run
```

https://github.com/thefireflyer/midterm/assets/64284477/7247472c-ae22-431d-a544-1f43caeac9f9



## Approach


## Review
