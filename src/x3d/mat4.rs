use std::ops;
use Vec3;
use std::f32;

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    v: [f32; 16],
}

fn idx(i: i32, j: i32) -> usize {
    (i + j * 4) as usize
}

impl Mat4 {
    pub fn new() -> Mat4 {
        Mat4 { v: [0.0; 16] }
    }

    pub fn from_vec(vec: &[f32; 16]) -> Mat4 {
        Mat4 { v: *vec }
    }

    pub fn identity() -> Mat4 {
        Mat4 {
            v: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0
            ],
        }
    }

    pub fn scale(scale: f32) -> Mat4 {
        Mat4 {
            v: [
                scale, 0.0, 0.0, 0.0, 0.0, scale, 0.0, 0.0, 0.0, 0.0, scale, 0.0, 0.0, 0.0, 0.0,
                scale,
            ],
        }
    }

    pub fn get(&self, i: i32, j: i32) -> f32 {
        self.v[(i + j * 4) as usize]
    }

    pub fn invert(&self) -> Mat4 {
        invert(self).unwrap_or_else(|| Mat4::identity())
    }

    pub fn rotate_x(angle: f32) -> Mat4 {
        let a = f32::consts::PI * angle / 180.0;
        Mat4::from_vec(&[
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            f32::cos(a),
            -f32::sin(a),
            0.0,
            0.0,
            f32::sin(a),
            f32::cos(a),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }

    pub fn rotate_y(angle: f32) -> Mat4 {
        let a = f32::consts::PI * angle / 180.0;
        Mat4::from_vec(&[
            f32::cos(a), // 0
            0.0,
            f32::sin(a),
            0.0,
            0.0, // 1
            1.0,
            0.0,
            0.0,
            -f32::sin(a), // 2
            0.0,
            f32::cos(a),
            0.0,
            0.0, // 3
            0.0,
            0.0,
            1.0,
        ])
    }

    pub fn rotate_z(angle: f32) -> Mat4 {
        let a = f32::consts::PI * angle / 180.0;
        Mat4::from_vec(&[
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            f32::cos(a),
            -f32::sin(a),
            0.0,
            0.0,
            f32::sin(a),
            f32::cos(a),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4::from_vec(&[
            1.0, // 0
            0.0,
            0.0,
            x,
            0.0, // 1
            1.0,
            0.0,
            y,
            0.0, // 2
            0.0,
            1.0,
            z,
            0.0, // 3
            0.0,
            0.0,
            1.0,
        ])
    }

    pub fn without_translate(&self) -> Mat4 {
        let mut r = Mat4::from_vec(&self.v);
        r.v[0 * 4 + 3] = 0.0;
        r.v[1 * 4 + 3] = 0.0;
        r.v[2 * 4 + 3] = 0.0;
        r.v[3 * 4 + 0] = 0.0;
        r.v[3 * 4 + 1] = 0.0;
        r.v[3 * 4 + 2] = 0.0;
        r.v[3 * 4 + 3] = 1.0;
        r
    }
}

impl ops::Add<Mat4> for Mat4 {
    type Output = Mat4;

    fn add(self, rhs: Mat4) -> Mat4 {
        let mut r = Mat4::new();
        for i in 0..16 {
            r.v[i] = self.v[i] + rhs.v[i]
        }
        r
    }
}

impl ops::Sub<Mat4> for Mat4 {
    type Output = Mat4;

    fn sub(self, rhs: Mat4) -> Mat4 {
        let mut v = [0.0; 16];
        for i in 0..16 {
            v[i] = self.v[i] - rhs.v[i]
        }
        Mat4 { v: v }
    }
}

impl ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        let mut v = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                let mut r = 0.0;
                for n in 0..4 {
                    r += self.get(n, j) * rhs.get(i, n)
                }
                v[idx(i, j)] = r
            }
        }
        Mat4 { v: v }
    }
}

impl ops::Div<f32> for Mat4 {
    type Output = Mat4;

    fn div(self, rhs: f32) -> Mat4 {
        let mut v = [0.0; 16];
        for i in 0..16 {
            v[i] = self.v[i] / rhs
        }
        Mat4 { v: v }
    }
}

impl ops::Mul<Vec3> for Mat4 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let mut r = Vec3::new();
        for n in 0..3usize {
            let mut x = 0.0;
            for i in 0..3usize {
                x += self.get(i as i32, n as i32) * rhs[i];
            }
            x += self.get(3, n as i32);
            r[n] = x
        }
        r
    }
}

fn det(mat: &Mat4) -> f32 {
    let m = &mat.v;

    m[0] * m[5] * m[10] * m[15] + m[0] * m[6] * m[11] * m[13] + m[0] * m[7] * m[9] * m[14]
        + m[1] * m[4] * m[11] * m[14] + m[1] * m[6] * m[8] * m[15] + m[1] * m[7] * m[10] * m[12]
        + m[2] * m[4] * m[9] * m[15] + m[2] * m[5] * m[11] * m[12] + m[2] * m[7] * m[8] * m[13]
        + m[3] * m[4] * m[10] * m[13] + m[3] * m[5] * m[8] * m[14] + m[3] * m[6] * m[9] * m[12]
        - m[0] * m[5] * m[11] * m[14] - m[0] * m[6] * m[9] * m[15] - m[0] * m[7] * m[10] * m[13]
        - m[1] * m[4] * m[10] * m[15] - m[1] * m[6] * m[11] * m[12] - m[1] * m[7] * m[8] * m[14]
        - m[2] * m[4] * m[11] * m[13] - m[2] * m[5] * m[8] * m[15] - m[2] * m[7] * m[9] * m[12]
        - m[3] * m[4] * m[9] * m[14] - m[3] * m[5] * m[10] * m[12] - m[3] * m[6] * m[8] * m[13]
}

fn invert(mat: &Mat4) -> Option<Mat4> {
    let det = det(mat);
    if det.abs() < f32::EPSILON {
        None
    } else {
        let m = &mat.v;
        let inv_det = 1.0 / det;
        let mut invm = [0.0; 16];

        invm[0] = inv_det
            * (m[5] * m[10] * m[15] + m[6] * m[11] * m[13] + m[7] * m[9] * m[14]
                - m[5] * m[11] * m[14] - m[6] * m[9] * m[15] - m[7] * m[10] * m[13]);
        invm[1] = inv_det
            * (m[1] * m[11] * m[14] + m[2] * m[9] * m[15] + m[3] * m[10] * m[13]
                - m[1] * m[10] * m[15] - m[2] * m[11] * m[13] - m[3] * m[9] * m[14]);
        invm[2] = inv_det
            * (m[1] * m[6] * m[15] + m[2] * m[7] * m[13] + m[3] * m[5] * m[14] - m[1] * m[7] * m[14]
                - m[2] * m[5] * m[15] - m[3] * m[6] * m[13]);
        invm[3] = inv_det
            * (m[1] * m[7] * m[10] + m[2] * m[5] * m[11] + m[3] * m[6] * m[9] - m[1] * m[6] * m[11]
                - m[2] * m[7] * m[9] - m[3] * m[5] * m[10]);

        invm[4] = inv_det
            * (m[4] * m[11] * m[14] + m[6] * m[8] * m[15] + m[7] * m[10] * m[12]
                - m[4] * m[10] * m[15] - m[6] * m[11] * m[12] - m[7] * m[8] * m[14]);
        invm[5] = inv_det
            * (m[0] * m[10] * m[15] + m[2] * m[11] * m[12] + m[3] * m[8] * m[14]
                - m[0] * m[11] * m[14] - m[2] * m[8] * m[15] - m[3] * m[10] * m[12]);
        invm[6] = inv_det
            * (m[0] * m[7] * m[14] + m[2] * m[4] * m[15] + m[3] * m[6] * m[12] - m[0] * m[6] * m[15]
                - m[2] * m[7] * m[12] - m[3] * m[4] * m[14]);
        invm[7] = inv_det
            * (m[0] * m[6] * m[11] + m[2] * m[7] * m[8] + m[3] * m[4] * m[10] - m[0] * m[7] * m[10]
                - m[2] * m[4] * m[11] - m[3] * m[6] * m[8]);

        invm[8] = inv_det
            * (m[4] * m[9] * m[15] + m[5] * m[11] * m[12] + m[7] * m[8] * m[13]
                - m[4] * m[11] * m[13] - m[5] * m[8] * m[15] - m[7] * m[9] * m[12]);
        invm[9] = inv_det
            * (m[0] * m[11] * m[13] + m[1] * m[8] * m[15] + m[3] * m[9] * m[12]
                - m[0] * m[9] * m[15] - m[1] * m[11] * m[12] - m[3] * m[8] * m[13]);
        invm[10] = inv_det
            * (m[0] * m[5] * m[15] + m[1] * m[7] * m[12] + m[3] * m[4] * m[13] - m[0] * m[7] * m[13]
                - m[1] * m[4] * m[15] - m[3] * m[5] * m[12]);
        invm[11] = inv_det
            * (m[0] * m[7] * m[9] + m[1] * m[4] * m[11] + m[3] * m[5] * m[8] - m[0] * m[5] * m[11]
                - m[1] * m[7] * m[8] - m[3] * m[4] * m[9]);

        invm[12] = inv_det
            * (m[4] * m[10] * m[13] + m[5] * m[8] * m[14] + m[6] * m[9] * m[12]
                - m[4] * m[9] * m[14] - m[5] * m[10] * m[12] - m[6] * m[8] * m[13]);
        invm[13] = inv_det
            * (m[0] * m[9] * m[14] + m[1] * m[10] * m[12] + m[2] * m[8] * m[13]
                - m[0] * m[10] * m[13] - m[1] * m[8] * m[14] - m[2] * m[9] * m[12]);
        invm[14] = inv_det
            * (m[0] * m[6] * m[13] + m[1] * m[4] * m[14] + m[2] * m[5] * m[12] - m[0] * m[5] * m[14]
                - m[1] * m[6] * m[12] - m[2] * m[4] * m[13]);
        invm[15] = inv_det
            * (m[0] * m[5] * m[10] + m[1] * m[6] * m[8] + m[2] * m[4] * m[9] - m[0] * m[6] * m[9]
                - m[1] * m[4] * m[10] - m[2] * m[5] * m[8]);

        Some(Mat4 { v: invm })
    }
}
