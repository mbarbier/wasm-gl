use js_sys::Math;

#[allow(dead_code)]
pub fn cube(width: f32, height: f32, depth: f32) -> (Vec<u16>, Vec<f32>, Vec<f32>) {
    #[rustfmt::skip]
    let indices = [0, 2, 1, 0, 3, 2, 4, 6, 5, 4, 7, 6, 8, 10, 9, 8, 11, 10, 12, 14, 13, 12, 15, 14, 16, 18, 17, 16, 19, 18, 20, 22, 21, 20, 23, 22];
    #[rustfmt::skip]
    let normals = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0] as [f32; 72];
    #[rustfmt::skip]
    let mut positions = [1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0] as [f32; 72];

    let scale = [width / 2.0, height / 2.0, depth / 2.0];
    positions.iter_mut().enumerate().for_each(|(idx, v)| {
        *v = scale[idx % 3] * *v;
    });

    // console_log!(format!("cube={:?}", positions));

    (indices.to_vec(), positions.to_vec(), normals.to_vec())
}

#[allow(dead_code)]
pub fn sphere(radius: f32) -> (Vec<u16>, Vec<f32>, Vec<f32>) {
    let mut positions: Vec<f32> = Vec::new();
    let mut normals: Vec<f32> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    let height_segments = 24;
    let width_segments = 24;

    let mut grid: Vec<Vec<u16>> = Vec::new();
    let mut index = 0;

    for iy in 0..(height_segments + 1) {
        let mut vertices_row: Vec<u16> = Vec::new();
        let v = iy as f32 / height_segments as f32;

        for ix in 0..(width_segments + 1) {
            let u = ix as f32 / width_segments as f32;

            let x = -radius * cos(u * PI * 2.0) * sin(v * PI);
            let y = radius * cos(v * PI);
            let z = radius * sin(u * PI * 2.0) * sin(v * PI);

            positions.push(x);
            positions.push(y);
            positions.push(z);

            normals.push(x);
            normals.push(y);
            normals.push(z);

            vertices_row.push(index);
            index = index + 1;
        }

        grid.push(vertices_row);
    }

    for iy in 0..height_segments {
        for ix in 0..width_segments {
            let a = grid[iy][ix + 1];
            let b = grid[iy][ix];
            let c = grid[iy + 1][ix];
            let d = grid[iy + 1][ix + 1];

            if iy != 0 {
                indices.push(a);
                indices.push(b);
                indices.push(d);
            }
            if iy != (height_segments - 1) {
                indices.push(b);
                indices.push(c);
                indices.push(d);
            }
        }
    }

    (indices, positions, normals)
}

pub fn cos(n: f32) -> f32 {
    Math::cos(n as f64) as f32
}
pub fn sin(n: f32) -> f32 {
    Math::sin(n as f64) as f32
}

pub const PI: f32 = 3.14159265359;
