pub fn translation(x: f32, y: f32, z: f32) -> [f32; 16] {
    [
       1.0, 0.0, 0.0, 0.0,
       0.0, 1.0, 0.0, 0.0,
       0.0, 0.0, 1.0, 0.0,
         x,   y,   z, 1.0,
    ]
}

pub fn rotation_x(degrees: f32) -> [f32; 16] {
    let c = f32::cos(degrees);
    let s = f32::sin(degrees);
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, c  , s  , 0.0,
        0.0, -s , c  , 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn rotation_y(degrees: f32) -> [f32; 16] {
    let c = f32::cos(degrees);
    let s = f32::sin(degrees);
    [
        c  , 0.0, -s , 0.0,
        0.0, 1.0, 0.0, 0.0,
        s  , 0.0, c  , 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn rotation_z(degrees: f32) -> [f32; 16] {
    let c = f32::cos(degrees);
    let s = f32::sin(degrees);
    [
          c,  -s, 0.0, 0.0,
          s,   c, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn scaling(x:f32, y:f32, z:f32) -> [f32; 16]{
    [
      x,  0.0,  0.0,  0.0,
      0.0,  y,  0.0,  0.0,
      0.0,  0.0,  z,  0.0,
      0.0,  0.0,  0.0,1.0,
    ]

}

