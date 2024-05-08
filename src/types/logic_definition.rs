
pub struct Display {
    material: String,

    coord_start: (f32, f32, f32),
    coord_end: (f32, f32, f32),
    rotation: (f32, f32, f32),

    base_rgba: (u8, u8, u8, u8),
    secondary_rgba: (u8, u8, u8, u8),
    enable_physics: Boolean,

    inputA: Vec<f32>,
    inputAType: String,
    logic_id: u16,
    decimals: u8
}

pub struct Math {
    material: String,

    operand: String,    // Add, Subtract, Multiply, Divide, 
                        // Fmod, Power, Greater, Less, Min,
                        // Max, Abs, Sign, Round, Ceil, Floor, 
                        // Sqrt, SinDeg, Sin, AsinDeg, Asin,
                        // CosDeg, Cos, AcosDeg, Acos, TanDeg,
                        // Tan, AtanDeg, Atan
    inputA: Vec<f32>,
    inputAType: String,
    inputB: Vec<f32>,
    inputBType: String,
    logic_id: u16,

    position: (f32, f32, f32),
    rotation: (f32, f32, f32),

    base_rgba: (u8, u8, u8, u8),
    enable_physics: Boolean,
}

pub struct Switch {
    material: String,

    inputA: Vec<f32>,
    inputAType: String,
    minIn: f32,
    minOut: f32,
    maxIn: f32,
    maxOut: f32,
    logic_id: u16,

    position: (f32, f32, f32),
    rotation: (f32, f32, f32),

    base_rgba: (u8, u8, u8, u8),
    enable_physics: Boolean,
}

pub struct Light1x1x1 {
    material: String, // cloudy glass, glass

    inputA: Vec<f32>,
    inputAType: String,
    logic_id: u16,

    position: (f32, f32, f32),
    rotation: (f32, f32, f32),

    base_rgba: (u8, u8, u8, u8),
    enable_physics: Boolean,
}