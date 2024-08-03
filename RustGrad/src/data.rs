mod utils;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    label: usize,
}

pub fn generate_data(rng: &mut impl Rng, n: usize) -> (Vec<Point>, Vec<Point>, Vec<Point>) {
    let mut points: Vec<Point> = Vec::with_capacity(n);

    for _ in 0..n {
        let x: f64 = rng.gen_range(-2.0..=2.0);
        let y: f64 = rng.gen_range(-2.0..=2.0);
        let label = if x < 0.0 {
            0
        } else if y < 0.0 {
            1
        } else {
            2
        };
        points.push(Point { x, y, label });
    }

    let split_index_1 = (n as f64 * 0.8) as usize;
    let split_index_2 = (n as f64 * 0.9) as usize;

    let (train_data, rest) = points.split_at(split_index_1);
    let (val_data, test_data) = rest.split_at(split_index_2 - split_index_1);

    (train_data.to_vec(), val_data.to_vec(), test_data.to_vec())
}