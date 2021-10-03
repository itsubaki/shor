use num::Complex;

pub type Qubit = Vec<Complex<f64>>;

pub type Gate = Vec<Vec<Complex<f64>>>;

#[derive(Debug)]
pub struct Q {
    qb: Qubit,
}

pub fn new() -> Q {
    return Q { qb: vec![] };
}

impl Q {
    pub fn new(&mut self, qb: Qubit) -> u32 {
        if self.qb.len() == 0 {
            self.qb = qb;
            return 0;
        }

        self.tensor_product(qb);
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
        let mut list = vec![];
        for i in 0..self.number_of_bit() {
            for j in 0..qb.len() {
                if i == qb[j] {
                    list.push(clone(&g));
                    continue;
                }
                list.push(id());
            }
        }

        let gg: Gate = tensor_product_list(&list);
        println!("gate: {:?}", gg);
    }

    fn tensor_product(&mut self, qb: Qubit) {
        let mut v = vec![];
        for i in 0..self.qb.len() {
            for j in 0..qb.len() {
                v.push(self.qb[i] * qb[j]);
            }
        }

        self.qb = v
    }
}

fn tensor_product(m: Gate, n: Gate) -> Gate {
    let mut out: Gate = vec![];
    for i in 0..m.len() {
        for k in 0..n.len() {
            let mut v: Vec<Complex<f64>> = vec![];
            for j in 0..m[i].len() {
                for l in 0..n[k].len() {
                    v.push(m[i][j] * n[k][l]);
                }
            }

            out.push(v);
        }
    }

    return out;
}

fn tensor_product_list(g: &[Gate]) -> Gate {
    let mut out = clone(&g[0]);
    for i in 1..g.len() {
        out = tensor_product(out, clone(&g[i]));
    }

    return out;
}

fn clone(g: &Gate) -> Gate {
    let mut out: Gate = vec![];
    for i in 0..g.len() {
        let mut v: Vec<Complex<f64>> = vec![];
        for j in 0..g[i].len() {
            v.push(g[i][j]);
        }

        out.push(v);
    }

    return out;
}

fn id() -> Gate {
    return vec![
        vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }],
        vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }],
    ];
}

fn x() -> Gate {
    return vec![
        vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }],
        vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }],
    ];
}

fn h() -> Gate {
    let e = Complex {
        re: 1.0 / std::f64::consts::SQRT_2,
        im: 0.0,
    };

    return vec![vec![e, e], vec![e, -1.0 * e]];
}
