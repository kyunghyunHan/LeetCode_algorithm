use std::collections::VecDeque;

struct MovingAverage {
    size: i32,
    queue: VecDeque<i32>,
    window_sum: i32,
}

impl MovingAverage {
    fn new( size: i32) -> Self {
        Self {
            size: size,
            queue: VecDeque::new(),
            window_sum: 0,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.queue.push_back(val);
        self.window_sum += val;
        if self.queue.len() as i32 > self.size{
             let removed = self.queue.pop_front().unwrap();
             self.window_sum  -= removed;
        }
        self.window_sum as f64 / self.queue.len() as f64


    }
}
