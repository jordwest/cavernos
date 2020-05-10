pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl Rect {
    pub fn width(&self) -> i32 {
        self.right - self.left + 1
    }
    pub fn height(&self) -> i32 {
        self.bottom - self.top + 1
    }

    pub fn consumeRows(&self, rows: i32) -> Rect {
        Rect {
            left: self.left,
            right: self.right,
            top: self.top + rows,
            bottom: self.bottom,
        }
    }

    pub fn pad(&self, top: i32, right: i32, bottom: i32, left: i32) -> Rect {
        Rect {
            top: self.top + top,
            left: self.left + left,
            right: self.right - right,
            bottom: self.bottom - bottom,
        }
    }
}
