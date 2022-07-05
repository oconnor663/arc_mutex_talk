# Arc and Mutex by Example
## Climbing the Learning Curve into Intermediate Rust

Threading is really central to what Rust is doing, but it takes a lot of
work to get up to speed to understand these examples.

you need a dozen different examples to explore all the different rules
that are coming into the picture here
- mutability
- lifetimes
- move semantics
- traits
- closures
later:
- static
- const
- atomic

Threading in Rust can be so difficult in Rust that it's demotivating.

It's especially hard to produce *working code* as a beginner, and it's a
lot easier to figure out why something doesn't work when you can compare
it to something that does work. So we're going to see a lot of working
code.

Also you really need hands-on experience to remember this stuff. You can
play with these examples.

And some people don't like just reading a book. It gets hard to keep
stuff in your head, and you need something specific to work towards.
Maybe understanding these examples can be something to work towards.
Also seeing how fast computers can do arithmetic is kind of fun and
worth playing around with.

- closure syntax
    - Like C++ lambda syntax, the Python `lambda` keyword, the Go
      `func` keyword, JS `function` or `=>`.
    - C++ and Rust both have to be careful with "capture by value"
      vs "capture by reference". Thus the `move` keyword.
    - EXERCISE: Can we remove the `move` keyword? No.
    - EXERCISE: Can we get rid of the Arc? No. Same lifetime error.
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
- [example 3](examples/3.rs): global Mutex, non-const function error
    - will be const in 1.63, but contents might not be
    - note that the closure goes away, regular functions impl FnOnce
- [example 4](examples/4.rs): unsafe global Mutex with Option
    - static mut is always unsafe
    - could've used MaybeUninit, or just a raw pointer
- [example 5](examples/5.rs): safe global Mutex with once\_cell::sync::Lazy
    - https://doc.rust-lang.org/std/lazy/struct.SyncLazy.html
- [example 6](examples/6.rs): crossbeam::scope also works
    - https://doc.rust-lang.org/std/thread/fn.scope.html
    - will be standard in 1.63
    - was also standard pre-1.0, but unsound
- [example 7](examples/7.rs): unsafe static mut u64
    - EXERCISE: doesn't work with --release
- [example 8](examples/8.rs): atomics
