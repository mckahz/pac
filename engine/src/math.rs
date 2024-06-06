pub struct V2 {
    pub x: f32,
    pub y: f32,
}

impl V2 {
    pub fn as_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}
