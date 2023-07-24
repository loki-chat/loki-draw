/// A simple rectangle.
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h
    }

    pub fn vflip(self) -> Rect {
        Self {
            x: self.x,
            y: self.y + self.h,
            w: self.w,
            h: -self.h,
        }
    }

    pub fn hflip(self) -> Rect {
        Self {
            x: self.x + self.w,
            y: self.y,
            w: -self.w,
            h: self.h,
        }
    }

    pub fn hvflip(self) -> Rect {
        self.vflip().hflip()
    }

    pub fn edge(self, edge: u32, size: f32) -> Self {
        match edge {
            0 => Self {
                x: self.x,
                y: self.y,
                w: self.w,
                h: size,
            },
            1 => Self {
                x: self.x + self.w,
                y: self.y,
                w: -size,
                h: self.h,
            },
            2 => Self {
                x: self.x,
                y: self.y + self.h,
                w: self.w,
                h: -size,
            },
            3 => Self {
                x: self.x,
                y: self.y,
                w: size,
                h: self.h,
            },
            _ => {
                panic!("unknown edge")
            }
        }
    }
}
