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
    pub fn new(&mut self, v: Qubit) -> u32 {
        if self.qb.len() == 0 {
            self.qb = v;
            return 0;
        }

        self.tensor_product(v);
        return self.number_of_bit() - 1;
    }

    pub fn zero(&mut self) -> u32 {
        return self.new(vec![
            Complex { re: 1.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
        ]);
    }

    pub fn zero_with(&mut self, n: u32) -> Vec<u32> {
        let mut out = vec![];
        for _ in 0..n {
            out.push(self.zero());
        }

        return out;
    }

    pub fn zero_log2(&mut self, n: u32) -> Vec<u32> {
        let s = ((n as f64).log2() as u32) + 1;
        return self.zero_with(s);
    }

    pub fn number_of_bit(&self) -> u32 {
        return (self.qb.len() as f64).log2() as u32;
    }

    pub fn x(&mut self, qb: &[u32]) {
        self.apply(x(), qb);
    }

    pub fn h(&mut self, qb: &[u32]) {
        self.apply(h(), qb);
    }

    pub fn cmodexp2(&mut self, a: u32, n: u32, r0: &[u32], r1: &[u32]) {}

    pub fn iqft(&mut self, qb: &[u32]) {}

    pub fn apply(&mut self, g: Gate, qb: &[u32]) {
        println!("{:?} {:?}", g, qb);
    }

    pub fn tensor_product(&mut self, qb: Qubit) {
        self.qb = qb
    }
}
