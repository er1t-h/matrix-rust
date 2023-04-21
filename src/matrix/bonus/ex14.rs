use crate::Matrix;

impl Matrix<f64> {
    pub fn projection(fov: f64, ratio: f64, near: f64, far: f64) -> Self {
        Matrix::from([
            [1.0 / (ratio * (fov / 2.0).tan()), 0.0, 0.0, 0.0],
            [0.0, 1.0 / (fov / 2.0).tan(), 0.0, 0.0],
            [
                0.0,
                0.0,
                -(far + near) / (far - near),
                -(2.0 * far * near) / (far - near),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ])
    }
}

mod test {
    use std::{fs::File, io::Write};

    use crate::Matrix;

    #[test]
    #[ignore = "Just prints the matrix to file"]
    fn example() {
        let mut file = File::create("proj").expect("Couldn't create the file");
        let mat = Matrix::<f64>::projection(80.0, 1.0, 1.0, 100.0);
        let mut str = String::new();
        for line in 0..4 {
            str += &mat
                .get_line(line)
                .unwrap()
                .fold(String::new(), |str, x| str + ", " + &x.to_string());
            str.push('\n');
        }
        write!(file, "{}", str).unwrap();
    }
}
