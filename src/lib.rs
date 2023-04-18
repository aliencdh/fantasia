use std::ops::{Add, Mul, Sub};

#[derive(Clone)]
pub struct Renderer {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
}
impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height * 4],
            width,
            height,
        }
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn clear(&mut self, color: Rgba) {
        self.buffer = std::iter::repeat(color)
            .flat_map(|color| color.bytes())
            .take(self.buffer.len())
            .collect();
    }

    pub fn line(&mut self, color: Rgba, from: Point, to: Point) {
        for t in 0..100 {
            let t = t as f32 / 100.;
            let current = t * (to - from);

            self.put_pixel(self.to_screen_coords(from + current), color);
        }
    }

    pub fn to_screen_coords(&self, point: Point) -> (usize, usize) {
        (
            ((point.x + 1.) * self.width as f32 / 2.) as usize,
            ((point.y + 1.) * self.height as f32 / 2.) as usize,
        )
    }

    pub fn put_pixel(&mut self, coords: (usize, usize), color: Rgba) {
        let idx = coords.0 + coords.1 * self.width;
        self.buffer
            .splice(idx..idx + 4, color.bytes().iter().cloned());
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn bytes(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}
impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}
impl Add for Point {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}
/// Scalar Multiplication
impl Mul<f32> for Point {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}
impl Mul<Point> for f32 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self
    }
}
