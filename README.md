# Rust After The Book: Arc, Mutex, and threading

- threading in Rust
- safe multithreading is a really important design goal for Rust
- a lot of language features get involved in threading code
    - lifetimes
    - mutability
    - move semantics
    - closures
    - traits like Deref, Send, and FnOnce
    - static and const
- it's a great way to learn but also challenging
- Rust books (and Rustlings) often include an Arc<Mutex<T>> example, but it's
  usually late in the book, and there isn't a lot of space to really *play*
  with the code.
- But Arc and Mutex are so central to what Rust is doing, that it's really
  worth sitting down with them and getting our hands dirty.
- So this is going to be an intermediate level talk, and I'm going
  to move kind of fast to get through lots of examples.

## [main.rs](src/main.rs)

- `Arc` is "like" a shared reference without a lifetime.
    - `number` and `alias` are pointing to the same thing.
    - Internally there's reference counting.
    - EXERCISE: Replace the `Arc` with a regular reference. Lifetime error.
    - EXERCISE: Get rid of alias entirely. Use of moved value.
    - EXERCISE: Get rid of the `move` keyword. Lifetime error again.

## [example 1](examples/1.rs): non-temporary guards

- MutexGuard is "like" a mutable reference.
- We need the explicit `drop()` because the guard is no longer temporary.
- EXERCISE: Forget to drop the guard. The Mutex is "unfair".
- EXERCISE: Try the fair Mutex from parking_lot.
- What does it mean to be "like" a reference? `Deref` and `DerefMut`.
- `Deref` happens in `println` and `DerefMut` happens with the `+=` operator.
- There's another deref hidden in `.lock()`.
- EXERCISE: Remove the `Mutex`. `Arc` doesn't implement `DerefMut`.

## [example 2](examples/2.rs): an `add_loop` function

- This is contrived, but common as `&String` -> `&str`.
- `thread::spawn`
    - `FnOnce`/`FnMut`/`Fn`
    - `'static`
    - `Send`
- EXERCISE: Use `Rc` instead of `Arc`. Closure must be `Send`.

## [example 3](examples/3.rs): `thread::scope`

- Note that `add_loop` is exactly the same as in example 2. The shared
  reference doesn't care where it comes from.
- A similar API was almost stabilized with Rust 1.0, but [turned out to be
  unsound](https://github.com/rust-lang/rust/issues/24292)

## [example 4](examples/4.rs): global `Mutex<u64>`

- Note that the closure goes away. Regular functions impl `FnOnce`.
- This didn't work prior to Rust 1.63.

## [example 5](examples/5.rs): global `Mutex<BigUint>`

- `Mutex::new` is const but `BigUint::from` isn't.

## [example 6](examples/6.rs): global `Mutex<Option<BigUint>>`

- This works but it's annoying.
- Initialization is error-prone, and tests are tricky.

## [example 7](examples/7.rs): global `Lazy<Mutex<BigUint>>`

- [`std::sync::LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html)
  will be stable someday.

## [example 8](examples/8.rs): unsound `static mut`

- EXERCISE: This breaks under `--release`.

## [example 9](examples/9.rs): `AtomicU64`
