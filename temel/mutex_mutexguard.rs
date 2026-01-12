use std::sync::{Mutex, MutexGuard};

#[derive(Debug)]
struct State {
    val: u32,
}

impl State {
    const fn new() -> Self {
        State { val: 0 }
    }
}

static GLOBSTATE: Mutex<State> = Mutex::new(State::new());

trait Glob<'a> {
    fn access(&'a self) -> MutexGuard<'a, State>;
    fn print(&'a self);
    fn incr(&'a self);
}

impl<'a> Glob<'a> for Mutex<State> {
    fn access(&'a self) -> MutexGuard<'a, State> {
        self.lock().unwrap()
    }

    fn print(&'a self) {
        let state = self.access();
        println!("{:?}", state);
    }

    fn incr(&'a self) {
        let mut state = self.access();
        state.val += 1;
    }
}

fn main() {
    GLOBSTATE.print();
    GLOBSTATE.incr();
    GLOBSTATE.print();
}
