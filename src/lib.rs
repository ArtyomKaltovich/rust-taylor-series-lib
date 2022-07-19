use std::cmp::Ordering;

pub fn exp(x: f64, delta: f64) -> f64 {
    match x.total_cmp(&0.0) {
        Ordering::Less => 1.0 / _exp(-x, delta),
        Ordering::Equal => 1.0,
        Ordering::Greater => _exp(x, delta),
    }
}

pub fn _exp(x: f64, delta: f64) -> f64 {
    let mut result = 1.0;
    let mut fact = 0.0;
    let mut step = 1.0;
    loop {
        if step < delta {
            return result;
        }
        fact = fact + 1.0;
        step *= x / fact;
        result += step;
    }
}
