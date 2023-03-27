# Arc and Mutex by Example
## Climbing the Learning Curve into Intermediate Rust

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

- Arc is "like" a shared reference with a 'static lifetime.
    - Like a shared reference means that `number` and `alias` are
      pointing to the same thing.
    - 'static means no borrowing. Internally there's reference
      counting.
    - EXERCISE: Can we get rid of the Arc? No, we get an error
      about non-static lifetimes.
    - `move` means that the closure is taking ownership.
    - EXERCISE: Can we remove the `move` keyword? No. It's the same
      error, this time because we have a shared reference to the
      Arc.
    - EXERCISE: Can we get rid of `alias`? No, use of moved value.
- Arc has two jobs, "acting like" a shared reference, and freeing
  its contents in drop().
    - How did we get a lifetime error if Arc has no lifetimes?
      Because the closure was implicitly taking a shared reference.
- [example 1](examples/1.rs): explicit types
- MutexGuard has two jobs
    - "act like" a mutable reference
    - release the lock in drop()
- Why did I add the explicit drop() here?
    - Holding the guard across sleep would be bad. Or we could've
      used curly braces.
    - When we combined all three lines, the guard was a temporary.
    - EXERCISE: forgetting the guard and the "unfair" Mutex.
- What does it mean to be "like" a reference? Deref/DerefMut
    - Deref in println and DerefMut with the += operator
    - EXERCISE: omitting the Mutex doesn't work
    - automatic deref in .lock()
- [example 2](examples/2.rs): deref coercion in add\_loop
    - this is contrived, but common as &String -> &str.
- thread::spawn
    - FnOnce/FnMut/Fn, briefly
    - 'static
        - An Arc is static, but without the `move` keyword we end
          up taking a reference to it anyway. We need both.
    - Send
        - EXERCISE: Rc doesn't work
- [example 3](examples/3.rs) and [example 4](examples/4.rs): global Mutex, non-const function error
    - const in 1.63, but contents might not be
    - note that the closure goes away, regular functions impl FnOnce
- [example 4](examples/4.rs): static BigUint
    - constructing a BigUint isn't const
- [example 5](examples/5.rs): static BigUint using Option
    - works but initialization is error-prone, tricky for tests
- [example 6](examples/6.rs): statis BigUint with once\_cell::sync::Lazy
    - https://doc.rust-lang.org/std/sync/struct.LazyLock.html
- [example 7](examples/7.rs): thread::scope also works
    - Note that `add_loop` is exactly the same as in example 2. The shared
      reference doesn't care where it comes from.
- [example 8](examples/8.rs): unsafe static mut u64
    - EXERCISE: doesn't work with --release
- [example 9](examples/9.rs): atomics
