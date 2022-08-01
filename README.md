# Arc and Mutex by Example
## Climbing the Learning Curve into Intermediate Rust

- threading in Rust
- safe multithreading is a really important design goal for Rust
- a lot of language features get involved in threading code
    - lifetimes
    - mutability
    - move semantics
    - closures
    - traits like Send and FnOnce
    - static and const
- it's a great way to learn but also challenging
- Rust books (and Rustlings) often include an Arc<Mutex<T>> example, but it's
  usually late in the book, and there isn't a lot of space to really *play*
  with the code.
- But Arc and Mutex are so central to what Rust is doing, that it's really
  worth sitting down with them and getting our hands dirty.
- So this is going to be an intermediate level talk.

- closure syntax
    - Like C++ lambda syntax, the Python `lambda` keyword, the Go
      `func` keyword, JS `function` or `=>`.
    - C++ and Rust both have to be careful with "capture by value"
      vs "capture by reference". Thus the `move` keyword.
    - EXERCISE: Can we remove the `move` keyword? No.
    - EXERCISE: Can we get rid of the Arc? No, same lifetime error, this time
      taking ownership of a reference.
    - EXERCISE: Can we get rid of `alias`? No, use of moved value.
- Arc has two jobs
    - "act like" a shared reference
    - free the object in drop()
    - similar to std::shared\_ptr or Python's reference counting
    - How did we get a lifetime error if Arc has no lifetimes?
      Because the closure was implicitly taking a shared reference.
- [example 1](examples/1.rs): explicit types
- MutexGuard has two jobs
    - "act like" a mutable reference
    - release the lock in drop()
    - similar to std::lock\_guard or Python's `with` statement
- Why did I add the explicit drop() here?
    - Holding the guard across sleep would be bad. Or we could've
      used curly braces.
    - When we combined all three lines, the guard was a temporary.
    - EXERCISE: forgetting the guard and the "unfair" Mutex.
- What does it mean to be "like" a reference? Deref/DerefMut
    - Deref in println and DerefMut with the += operator
    - automatic deref in .lock()
- [example 2](examples/2.rs): deref coercion in add\_loop
    - this is contrived, but common as &String -> &str.
- EXERCISE: omitting the Mutex doesn't work
- thread::spawn
    - FnOnce/FnMut/Fn
        - self vs &mut self vs &self
        - NOT ENOUGH TIME FOR THIS
    - 'static
        - move keyword is required
        - EXAMPLE getting rid of the Arc doesn't work
    - Send
        - Rc doesn't work
- [example 3](examples/3.rs) and [example 4](examples/4.rs): global Mutex, non-const function error
    - const in 1.63, but contents might not be
    - note that the closure goes away, regular functions impl FnOnce
- [example 5](examples/5.rs): unsafe global Mutex with Option
    - static mut is always unsafe
    - could've used MaybeUninit, or just a raw pointer
- [example 6](examples/6.rs): safe global Mutex with once\_cell::sync::Lazy
    - https://doc.rust-lang.org/std/lazy/struct.SyncLazy.html
- [example 7](examples/7.rs): crossbeam::scope also works
    - https://doc.rust-lang.org/std/thread/fn.scope.html
    - will be standard in 1.63
    - was also standard pre-1.0, but unsound
- [example 8](examples/8.rs): unsafe static mut u64
    - EXERCISE: doesn't work with --release
- [example 9](examples/9.rs): atomics
