use crate::Matrix;

impl Matrix<f64> {
    pub fn projection(fov: f64, ratio: f64, near: f64, far: f64) -> Self {
        Self::from([
            [1.0 / (ratio * (fov / 2.0).tan()), 0.0, 0.0, 0.0],
            [0.0, 1.0 / (fov / 2.0).tan(), 0.0, 0.0],
            [
                0.0,
                0.0,
                -((far + near) / (far - near)),
                -((2.0 * far * near) / (far - near)),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ])
    }
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Write};

    use crate::Matrix;

    #[test]
    #[ignore = "Just prints the matrix to file"]
    fn example() {
        let fov: f64 = 60.;
        let fov_rad = fov.to_radians();
        let mut file = File::create("matrix_display/proj").expect("Couldn't create the file");
        let mat = Matrix::<f64>::projection(fov_rad, 1.0, 5.0, 50.0);
        let mut str = String::new();
        for line in 0..4 {
            str += &mat
                .get_line(line)
                .unwrap()
                .skip(1)
                .fold(mat.get(line, 0).unwrap().to_string(), |str, x| {
                    str + ", " + &x.to_string()
                });
            str.push('\n');
        }
        write!(file, "{str}").unwrap();
    }
}
