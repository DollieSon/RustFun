const EXP: f64 = 2.718281828459045;

pub fn sigmoid(x: &f64) -> f64 {
    return 1.0 / (1.0 + EXP.powf(-x));
}
