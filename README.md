# Intermediate Rust: Happy Accidents with Arc and Mutex

- closure syntax
    - Like C++ lambda syntax, the Python `lambda` keyword, the Go
      `func` keyword, JS `function` or `=>`.
    - C++ and Rust both have to be careful with "capture by value"
      vs "capture by reference". Thus the `move` keyword.
- [example 1]: explicit types
- MutexGuard has two jobs
    - "act like" a mutable reference
    - release the lock in drop()
    - similar to std::lock\_guard or Python's `with` statement
- Arc has two jobs
    - "act like" a shared reference
    - free the object in drop()
    - similar to std::shared\_ptr or Python's reference counting
- Why did I add the explicit drop() here?
    - Holding the guard across sleep would be bad. Or we could've
      used curly braces.
    - When we combined all three lines, the guard was a temporary.
    - EXERCISE: forgetting the guard and the "unfair" Mutex.
- Deref/DerefMut
    - Deref in println and DerefMut with the += operator
    - automatic deref in .lock()
- [example 2]: deref coercion in add\_loop
    - this is contrived, but common as &String -> &str.
- EXERCISE: omitting the Mutex doesn't work
- thread::spawn
    - FnOnce/FnMut/Fn
        - self vs &mut self vs &self
    - 'static
        - move keyword is required
        - EXAMPLE getting rid of the Arc doesn't work
    - Send
        - Rc doesn't work
- EXERCISE: omitting the Arc doesn't work
    - use-after-move, or non-static lifetime errors
- [example 3]: global Mutex, non-const function error
    - will be const in 1.63, but contents might not be
    - note that the closure goes away, regular functions impl FnOnce
- [example 4]: unsafe global Mutex with Option
    - static mut is always unsafe
    - could've used MaybeUninit, or just a raw pointer
- [example 5]: safe global Mutex with once\_cell::sync::Lazy
    - https://doc.rust-lang.org/std/lazy/struct.SyncLazy.html
- [example 6]: crossbeam::scope also works
    - https://doc.rust-lang.org/std/thread/fn.scope.html
    - will be standard in 1.63
    - was also standard pre-1.0, but unsound
- [example 7]: unsafe static mut u64
    - EXERCISE: doesn't work with --release
- [example 8]: atomics
