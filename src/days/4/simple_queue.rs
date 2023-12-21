pub trait SimpleQueue<T> {
    fn add(&mut self, t: T);
    fn next(&mut self) -> Option<T>;
}

struct VecWrapper<T> {
    v: Vec<T>,
}

impl<T> SimpleQueue<T> for VecWrapper<T> {
    fn add(&mut self, t: T) {
        self.v.push(t)
    }

    fn next(&mut self) -> Option<T> {
        self.v.pop()
    }
}

pub fn simple_queue<T>() -> impl SimpleQueue<T> {
    VecWrapper { v: vec![] }
}
