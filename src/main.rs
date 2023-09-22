use reentrancy::CallbacksMut;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
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
    // the "original" container (i.e. the data held pointed to by `rc2`).
    let mut c = rc.borrow().clone();
    drop(rc);
    c.call(1);
    // If the mutable borrow of closure 2 did not trigger an error, then the
    // re-borrow of `rc2` would; this is analogous to triggering via
    // `rc.borrow_mut().call(1)`.
}
