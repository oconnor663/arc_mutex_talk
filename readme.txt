- either be very experienced with C++, or have a few solid hours studying Rust
  - references, Vec, Option and Result, traits
  - also I'm going to assume you know what a thread is
- saw Rc<RefCell<T>> on leetcode, not a big fan of that
  - https://leetcode.com/explore/interview/card/top-interview-questions-medium/108/trees-and-graphs/786/

- Basic example with two threads.
  - Mutex is not fair on my machine. If we forget to drop() the guard, one
    thread gets starved.
  - `output_clone` is required, because the Arc is moved into the lambda.
  - The closure doesn't need `move` because moving is inferred.
  - Inlining `a` and `b` actually fails the borrow checker. But it works if we
    dereference `v` first. Turns out to be a limitation related to Deref and a
    special case in the borrow checker.

- Arc
  - Deref
  - Clone
  - Send & Sync
- Mutex
  - DerefMut
  - MutexGuard is Sync but not Send

- implement a tree with Option<Arc<Mutex<T>>> and then with Option<Box>
  - reread https://rust-unofficial.github.io/too-many-lists/
  - first with cloning
  - then without cloning
  - then without Arc

- also mention the IntoIterator magic in the for-loop
  - and the fact that Clone, Vec, and IntoIterator are in the prelude

- mention the lifetime of unbound temporaries

Magic list:
- the prelude
- Deref and DerefMut
- IntoIterator
- Send and Sync

Things that don't work:
- putting an Rc in the closure
- no Mutex
- mutex on the stack
  - but this works with `crossbeam` today, and it'll be in std in the future
  - https://doc.rust-lang.org/nightly/std/thread/fn.scope.html
- global Mutex
  - but this works with `once_cell` today, and again it'll be in std in the future
  - https://doc.rust-lang.org/nightly/std/lazy/struct.SyncLazy.html
