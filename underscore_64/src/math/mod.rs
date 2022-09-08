use crate::data::List;
use core::intrinsics::sinf32;

pub trait Vector {}

impl Vector for [f32; 2] {}

pub trait Curve<Domain, Image: Vector> {
    fn plot(&self, start: Domain, end: Domain, n: usize) -> List<Image>;
}

pub fn sin(x: f32) -> f32 {
    unsafe { sinf32(x) }
}

impl<F> Curve<f32, [f32; 2]> for F
where
    F: Fn(f32) -> f32,
{
    fn plot(&self, start: f32, end: f32, n: usize) -> List<[f32; 2]> {
        let dx = (end - start) / n as f32;
        let mut curve = List::new(n + 1);

        for point in (0..=n).map(|x| {
            let x = x as f32;
            [start + (x * dx), self(start + (x * dx))]
        }) {
            curve.push(point);
        }
        curve
    }
}

#[test]
fn plot_sin() {
    let start = -1.0;
    let end = 1.0;
    let n = 4;
    let sin_curve = sin.plot(start, end, n);

    assert_eq!(sin_curve.len(), 5);

    let dx = (end - start) / n as f32;
    for n in 0..=4 {
        let x = n as f32;
        assert_eq!(sin_curve[n], [start + (x * dx), sin(start + (x * dx))]);
    }
}
