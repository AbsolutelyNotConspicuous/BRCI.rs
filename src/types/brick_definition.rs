
pub struct NonScalable {
    material: String,
    position: (f32, f32, f32),
    rotation: (f32, f32, f32),
    base_rgba: (u8, u8, u8, u8),
    enable_physics: Boolean,
}

pub struct Scalable {
    material: String,
    coord_start: (f32, f32, f32),
    coord_end: (f32, f32, f32),
    rotation: (f32, f32, f32),
    base_rgba: (u8, u8, u8, u8),
    enable_physics: Boolean,
}
