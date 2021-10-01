use num::Complex;

pub type Qubit = Vec<Complex<f64>>;

pub type Gate = Vec<Vec<Complex<f64>>>;

#[derive(Debug)]
pub struct Q {
    qb: Qubit,
}

impl Q {
    pub fn new() -> Q {
        return Q { qb: vec![] };
    }

    pub fn zero(&mut self) -> i32 {
        self.qb = vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }];
        return 1;
    }

    pub fn x(&mut self, qb: i32) {
        let g: Gate = vec![
            vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }],
            vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }],
        ];

        self.apply(g, qb);
    }

    pub fn apply(&mut self, g: Gate, qb: i32) {
        println!("{:?} {}", g, qb);
    }
}
