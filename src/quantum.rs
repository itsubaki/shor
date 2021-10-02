use num::Complex;

pub type Gate = Vec<Vec<Complex<f64>>>;

fn x() -> Gate {
    return vec![
        vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }],
        vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }],
    ];
}

fn h() -> Gate {
    let e = Complex {
        re: 1.0 / 2.0f64.sqrt(),
        im: 0.0,
    };

    return vec![vec![e, e], vec![e, -1.0 * e]];
}

pub type Qubit = Vec<Complex<f64>>;

#[derive(Debug)]
pub struct Q {
    qb: Qubit,
}

pub fn new() -> Q {
    return Q { qb: vec![] };
}

impl Q {
    pub fn new(&mut self, v: Qubit) -> i32 {
        if self.qb.len() == 0 {
            self.qb = v;
            return 0;
        }

        self.tensor_product(v);
        return self.number_of_bit() - 1;
    }

    pub fn zero(&mut self) -> i32 {
        return self.new(vec![
            Complex { re: 1.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
        ]);
    }

    pub fn zero_with(&mut self, n: i32) -> Vec<i32> {
        let mut out = vec![];
        for _ in 0..n {
            out.push(self.zero());
        }

        return out;
    }

    pub fn zero_log2(&mut self, n: i32) -> Vec<i32> {
        let s = ((n as f64).log2() as i32) + 1;
        return self.zero_with(s);
    }

    pub fn number_of_bit(&self) -> i32 {
        return (self.qb.len() as f64).log2() as i32;
    }

    pub fn x(&mut self, qb: &[i32]) {
        self.apply(x(), qb);
    }

    pub fn h(&mut self, qb: &[i32]) {
        self.apply(h(), qb);
    }

    pub fn cmodexp2(&mut self, a: i32, n: i32, r0: &[i32], r1: &[i32]) {}

    pub fn iqft(&mut self, qb: &[i32]) {}

    pub fn apply(&mut self, g: Gate, qb: &[i32]) {
        println!("{:?} {:?}", g, qb);
    }

    pub fn tensor_product(&mut self, qb: Qubit) {
        self.qb = qb
    }
}
