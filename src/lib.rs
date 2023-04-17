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
        // Since we're using RGBA, we can expect every pixel to take up 4 bytes.
        self.buffer = std::iter::repeat(color)
            .flat_map(|color| color.bytes())
            .take(self.buffer.len())
            .collect();
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
