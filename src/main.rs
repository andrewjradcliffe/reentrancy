use reentrancy::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn internal() {
    let mut count: usize = 0;
    let mut c1 = CallbacksMut::new();
    c1.register(move |val| {
        count += 1;
        println!("Callback 1: {} ({}.time)", val, count);
    });
    let rc = Rc::new(RefCell::new(c1));
    let rc2 = Rc::clone(&rc);
    rc.borrow_mut().register(move |val| {
        println!("Callback 2: {}", val);
        let mut guard = rc2.borrow_mut();
        guard.call(val + val);
    });
    // Properly reentrant invocation. Irrespective of whether we drop `rc`,
    // when we `c.call(1)`, the original copy of the pointers exists. A second
    // mutable borrow occurs on the second call of closure 2, which happens inside
    // the "original" container (i.e. the data pointed to by `rc2`).
    let mut c = rc.borrow().clone();
    drop(rc);
    c.call(1);
    // If the mutable borrow of closure 2 did not trigger an error, then the
    // re-borrow of `rc2` would; this is analogous to triggering via
    // `rc.borrow_mut().call(1)`.
}

fn external() {
    // This illustrates the case in which calling of closures does not require
    // a mutable borrow. The structure mimics the internal reentrancy case,
    // except the enclosed environment is not mutable. When we `c.call(1)`,
    // closure 1 executes, then closure 2 mutably borrows the "original" container
    // (i.e. the data pointed to by `rc2`), thereby initiating a call of closure 1,
    // which occurs without issue, then a call of closure 2, which executes
    // without issue until a second mutable borrow of data pointed to by `rc2`
    // is attempted.
    // We encounter an error due only to the fact that we take a mutable borrow,
    // rather than immutable borrow. If we replace `rc2.borrow_mut()` with `rc2.borrow()`,
    // then we have infinite recursion instead.
    let count: Cell<usize> = Cell::new(0);
    let mut c1 = Callbacks::new();
    c1.register(move |val| {
        let current = count.get();
        count.set(current + 1);
        println!("Callback 1: {} ({}.time)", val, count.get());
    });
    let rc = Rc::new(RefCell::new(c1));
    let rc2 = Rc::clone(&rc);
    rc.borrow_mut().register(move |val| {
        println!("Callback 2: {}", val);
        // BorrowMutError
        let guard = rc2.borrow_mut();
        // Infinite recursion
        // let guard = rc.borrow();
        guard.call(val + val);
    });
    let c = rc.borrow().clone();
    drop(rc);
    c.call(1);
    // In contrast to the BorrowMutError demonstrated in `internal`,
    // this form of reentrancy should be apparent to the person which wrote
    // the code -- or, at least, the cause of the error is due the library user,
    // not the library writer. The internal example would be non-obvious,
    // and, in order to understand the cause, the library user would need
    // to consider some of the library internals.
}

fn fib_iter(n: i32) {
    // An example with recursive calls which is intended to terminate.
    // May as well permit Fibonacci in both directions, as this simplifies
    // questions about whether the program terminates
    // N.B. the stack will eventually overflow as this is recursive,
    // but i64 will underflow/overflow well before the stack does.
    let a: Cell<i64> = Cell::new(1);
    let b: Cell<i64> = Cell::new(0);
    let count: Cell<usize> = Cell::new(0);
    let mut c1 = Callbacks::new();
    // This is the loop body
    c1.register(move |i| {
        count.set(count.get() + 1);
        let t_b = b.get();
        if i >= 0 {
            let t = a.get() + t_b;
            b.set(a.get());
            a.set(t);
        } else {
            let t = a.get() - t_b;
            a.set(t_b);
            b.set(t);
        }
        println!(
            "Callback 1: Fib({}) = {} ({}.time)",
            n - i,
            t_b,
            count.get()
        );
    });
    let rc = Rc::new(RefCell::new(c1));
    let rc2 = Rc::clone(&rc);
    // This is the loop condition, albeit, applied after the loop body.
    rc.borrow_mut().register(move |i| {
        println!("Callback 2: {}", i);
        if i > 0 {
            let guard = rc2.borrow();
            guard.call(i - 1);
        } else if i < 0 {
            let guard = rc2.borrow();
            guard.call(i + 1);
        }
    });
    let c = rc.borrow().clone();
    drop(rc);
    c.call(n);
}

fn main() {
    // internal();
    // external();
    fib_iter(-25);
}
