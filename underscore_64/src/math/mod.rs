use crate::data::List;
use core::intrinsics::{powf32, sinf32};

pub type Spline = List<Bezier>;

impl Spline {
    pub fn points(&self) -> List<[f32; 2]> {
        self.iter()
            .fold(List::new(self.len()), |mut points, curve| {
                for &point in curve.plot(0.0, 1.0, 100).iter() {
                    points.push(point);
                }

                points
            })
    }
}

pub type Bezier = List<[f32; 2]>;

impl Curve<f32, [f32; 2]> for Bezier {
    fn plot(&self, start: f32, end: f32, n: usize) -> List<[f32; 2]> {
        (0..=n)
            .map(|x| {
                let t = (x as f32 / n as f32) * (end - start) + start;
                let mut point = [0.0, 0.0];
                let k = self.len() - 1;

                for v in 0..=k {
                    let b = factorial(k) as f32 / (factorial(v) * factorial(k - v)) as f32
                        * powf(t, v as f32)
                        * powf(1.0 - t, (k - v) as f32);

                    point[0] += b * self[v][0];
                    point[1] += b * self[v][1];
                }

                point
            })
            .collect()
    }
}

pub trait Curve<Domain, Image> {
    fn plot(&self, start: Domain, end: Domain, n: usize) -> List<Image>;
}

pub fn powf(a: f32, b: f32) -> f32 {
    unsafe { powf32(a, b) }
}

pub fn sin(x: f32) -> f32 {
    unsafe { sinf32(x) }
}

pub fn factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => factorial(n - 1) * n,
    }
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

#[test]
fn list_of_list() {
    assert_eq!(core::mem::size_of::<List<[f32; 2]>>(), 24);
    let mut outer: List<List<[f32; 2]>> = List::new(1);
    outer.push(List::new(1));

    assert_eq!(outer.len(), 1);
    assert_eq!(outer[0].len(), 0);
}
