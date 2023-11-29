pub fn rotation_x(degrees: f32) -> [f32; 16] {
    let c = f32::cos(degrees);
    let s = f32::sin(degrees);
    return [
        1.0, 0.0, 0.0, 0.0,
        0.0, c  , s  , 0.0,
        0.0, -s , c  , 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
}

pub fn rotation_y(degrees: f32) -> [f32; 16] {
    let c = f32::cos(degrees);
    let s = f32::sin(degrees);
    return [
        c  , 0.0, -s , 0.0,
        0.0, 1.0, 0.0, 0.0,
        s  , 0.0, c  , 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
}
