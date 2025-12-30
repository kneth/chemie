use std::env;

pub trait Field {
    fn new(p: Vec<f64>) -> impl Field;
    fn df(&self, x: &Vec<f64>) -> Vec<f64>;
}

fn solver(end_time: f64, field: impl Field, x0: Vec<f64>, print_time: f64) {
    let mut time = 0.0;
    let mut next_print_time = print_time;
    let dt = 0.001;
    let mut x: Vec<f64> = x0;
    while time <= end_time {
        let dx = field.df(&x);
        for i in 0..x.len() {
            x[i] = x[i] + dx[i]*dt;
        }
        time += dt;
        if time >= next_print_time {
            print!("{:?} ", time);
            for v in &x {
                print!("{} ", v);
            }
            println!("");
            next_print_time += print_time;
        }
    }
}

pub struct Lorentz {
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64
}

impl Field for Lorentz {
    fn new(p: Vec<f64>) -> impl Field {
        Lorentz {
            sigma: p[0],
            rho: p[1],
            beta: p[2]
        }
    }

    fn df(&self, x: &Vec<f64>) -> Vec<f64> {
        let d0 = self.sigma*(x[1]-x[0]);
        let d1 = x[0]*(self.rho-x[2]) - x[1];
        let d2 = x[0]*x[1] - self.beta*x[2];
        return vec![d0, d1, d2];
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
    let args: Vec<String> = env::args().collect();
    let schema = &args[1];

    match schema.as_str() {
        "brusselator" => { 
            let b = Brusselator::new(vec![1.0, 3.0]);
            solver(30.0, b, vec![1.0, 1.0], 0.1);
        }
        "lorentz" => {
            let l = Lorentz::new(vec![10.0, 28.0, 8.0/3.0]);
            solver(100.0, l, vec![0.9, 0.0, 0.0], 0.1);
        }
        _ => println!("Unknown schema")
    }
}
