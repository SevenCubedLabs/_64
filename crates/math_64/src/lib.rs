#![no_std]
#![feature(core_intrinsics)]
use {
    core::intrinsics::{powf32, sinf32},
    mem_64::Vec,
};

pub trait Points {
    fn points(&self) -> Vec<&[f32; 2]>;
}

pub type Matrix<const M: usize, const N: usize> = [[f32; N]; M];

pub fn ortho([left, bottom]: [f32; 2], [right, top]: [f32; 2]) -> Matrix<4, 4> {
    [
        [2.0 / (right - left), 0.0, 0.0, 0.0],
        [0.0, 2.0 / (top - bottom), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [
            -(right + left) / (right - left),
            -(top + bottom) / (top - bottom),
            0.0,
            1.0,
        ],
    ]
}

pub type Spline = Vec<Bezier>;

impl Points for Spline {
    fn points(&self) -> Vec<&[f32; 2]> {
        self.iter().flat_map(|bezier| bezier.0.iter()).collect()
    }
}

pub struct Bezier(Vec<[f32; 2]>);

impl From<&[[f32; 2]]> for Bezier {
    fn from(src: &[[f32; 2]]) -> Self {
        Bezier(src.into())
    }
}

impl Bezier {
    fn subdivide(&self, n: usize) -> Vec<[f32; 2]> {
        (0..=n)
            .map(|x| {
                let t = x as f32 / n as f32;
                let mut point = [0.0, 0.0];
                let k = self.0.len() - 1;

                for v in 0..=k {
                    let b = factorial(k) as f32 / (factorial(v) * factorial(k - v)) as f32
                        * powf(t, v as f32)
                        * powf(1.0 - t, (k - v) as f32);

                    point[0] += b * self.0[v][0];
                    point[1] += b * self.0[v][1];
                }

                point
            })
            .collect()
    }
}

pub trait Curve<Domain, Image> {
    fn plot(&self, start: Domain, end: Domain, n: usize) -> Vec<Image>;
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
    fn plot(&self, start: f32, end: f32, n: usize) -> Vec<[f32; 2]> {
        let dx = (end - start) / n as f32;
        let mut curve = Vec::with_capacity(n + 1);

        for point in (0..=n).map(|x| {
            let x = x as f32;
            [start + (x * dx), self(start + (x * dx))]
        }) {
            curve.push(point);
        }
        curve
    }
}
