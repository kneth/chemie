pub trait Field {
    fn new(p: Vec<f64>) -> impl Field;
    fn df(&self, x: &Vec<f64>) -> Vec<f64>;
}

fn solver(field: impl Field, x0: Vec<f64>) {
    let dt = 0.001;
    let mut x: Vec<f64> = Vec::new();
    x = x0;
    for i in 0..100000 {
        let dx = field.df(&x);
        for j in 0..x.len() {
            x[j] = x[j] + dx[j]*dt;
        }
        println!("{:?}, {:?}", (i as f64)*dt, x);
    }
}

pub struct Brusselator {
    pub a: f64,
    pub b: f64
}

impl Field for Brusselator {
    fn new(p: Vec<f64>) -> impl Field {
        Brusselator {
            a: p[0],
            b: p[1]
        }
    }

    fn df(&self, x: &Vec<f64>) -> Vec<f64> {
        let d0 = self.a + x[0]*x[0]*x[1] - self.b*x[0] - x[0];
        let d1 = self.b*x[0] - x[0]*x[0]*x[1];
        return vec![d0, d1];
    }
}


fn main() {
    let b = Brusselator::new(vec![1.0, 3.0]);
    solver(b, vec![1.0, 1.0]);
}
