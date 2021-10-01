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
        self.tensor_product(vec![
            Complex { re: 1.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
        ]);
        return self.number_of_bit() - 1;
    }

    pub fn zero_with(&mut self, n: i32) -> Vec<i32> {
        let mut out = vec![];
        for _ in 0..n {
            out.push(self.zero());
        }

        return out;
    }

    pub fn zero_log2(&mut self, n: i32) -> Vec<i32> {
        return vec![0];
    }

    pub fn x(&mut self, qb: &[i32]) {
        let g = vec![
            vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }],
            vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }],
        ];

        self.apply(g, qb);
    }

    pub fn h(&mut self, qb: &[i32]) {
        let e = Complex {
            re: 1.0 / 2.0f64.sqrt(),
            im: 0.0,
        };
        let g = vec![vec![e, e], vec![e, -1.0 * e]];

        self.apply(g, qb);
    }

    pub fn cmodexp2(&mut self, a: i32, n: i32, r0: &[i32], r1: &[i32]) {}

    pub fn iqft(&mut self, qb: &[i32]) {}

    pub fn apply(&mut self, g: Gate, qb: &[i32]) {
        println!("{:?} {:?}", g, qb);
    }

    pub fn number_of_bit(&self) -> i32 {
        return 1;
    }

    pub fn tensor_product(&mut self, qb: Qubit) {
        self.qb = qb
    }
}
