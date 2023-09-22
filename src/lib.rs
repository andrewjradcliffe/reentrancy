use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct CallbacksMut {
    callbacks: Vec<Rc<RefCell<dyn FnMut(i32)>>>,
}
impl CallbacksMut {
    pub fn new() -> Self {
        CallbacksMut {
            callbacks: Vec::new(),
        }
    }

    pub fn register<F: FnMut(i32) + 'static>(&mut self, callback: F) {
        let cell = Rc::new(RefCell::new(callback));
        self.callbacks.push(cell);
    }

    pub fn call(&mut self, val: i32) {
        for callback in self.callbacks.iter() {
            let mut closure = callback.borrow_mut();
            closure(val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
