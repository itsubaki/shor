use num::Complex;
use std::rc::Rc;

pub type Matrix = Vec<Vec<Complex<f64>>>;

pub type Qubit = Vec<Complex<f64>>;

pub type Gate = Rc<Matrix>;

pub struct State {
    n: usize,
    pub index: usize,
    pub amp: Complex<f64>,
    pub prob: f64,
}

impl std::fmt::Display for State {
    fn fmt(&self, dest: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            dest,
            "{:>0n$b} {:>.4} {:>.4}",
            self.index,
            self.amp,
            self.prob,
            n = self.n,
        )
    }
}

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

        self.tensor_vec(qb);
        return self.number_of_bit() - 1;
    }

    pub fn zero(&mut self) -> u32 {
        return self.new(vec![
            Complex { re: 1.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
        ]);
    }

    pub fn zero_with(&mut self, n: u32) -> Vec<u32> {
        let mut out: Vec<u32> = vec![];

        for _ in 0..n {
            out.push(self.zero());
        }

        return out;
    }

    pub fn zero_log2(&mut self, n: u32) -> Vec<u32> {
        let s = ((n as f64).log2() as u32) + 1;
        return self.zero_with(s);
    }

    fn tensor_vec(&mut self, qb: Qubit) {
        let mut v: Qubit = vec![];

        for i in 0..self.qb.len() {
            for j in 0..qb.len() {
                v.push(self.qb[i] * qb[j]);
            }
        }

        self.qb = v
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

    pub fn apply(&mut self, g: Gate, qb: &[u32]) {
        let list: Vec<Gate> = self.gate_list(g, id(), qb);
        let g: Gate = tensor_(&list);

        self.qb = apply(g, &self.qb);
    }

    pub fn cmodexp2(&mut self, a: u32, n: u32, r0: &[u32], r1: &[u32]) {
        println!("cmodexp2({}, {}, {:?}, {:?})", a, n, r0, r1);
    }

    pub fn iqft(&mut self, qb: &[u32]) {
        println!("iqft({:?})", qb);
    }

    fn gate_list(&self, g: Gate, id: Gate, qb: &[u32]) -> Vec<Gate> {
        let mut list: Vec<Gate> = vec![];

        for i in 0..self.number_of_bit() {
            let mut found = false;

            for j in 0..qb.len() {
                if i == qb[j] {
                    found = true;
                    break;
                }
            }

            if found {
                list.push(Rc::clone(&g));
                continue;
            }

            list.push(Rc::clone(&id));
        }

        return list;
    }

    pub fn state(&self) -> Vec<State> {
        let z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
        let n: usize = self.number_of_bit() as usize;

        let mut out: Vec<State> = vec![];

        for i in 0..self.qb.len() {
            if self.qb[i] == z {
                continue;
            }

            out.push(State {
                n: n,
                index: i,
                amp: self.qb[i],
                prob: self.qb[i].norm().powf(2.0),
            });
        }

        return out;
    }
}

fn apply(g: Gate, qb: &Qubit) -> Qubit {
    let mut out: Qubit = vec![];

    for i in 0..g.len() {
        let mut r = Complex { re: 0.0, im: 0.0 };

        for j in 0..g[i].len() {
            r = r + g[i][j] * qb[j]
        }

        out.push(r);
    }

    return out;
}

fn tensor_(list: &[Gate]) -> Gate {
    let mut g: Gate = Rc::clone(&list[0]);

    for i in 1..list.len() {
        g = tensor(g, Rc::clone(&list[i]));
    }

    return g;
}

fn tensor(m: Gate, n: Gate) -> Gate {
    let mut out: Matrix = vec![];

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

    return Rc::new(out);
}

fn id() -> Gate {
    return Rc::new(vec![
        vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }],
        vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }],
    ]);
}

fn x() -> Gate {
    return Rc::new(vec![
        vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }],
        vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }],
    ]);
}

fn h() -> Gate {
    let e = Complex {
        re: 1.0 / std::f64::consts::SQRT_2,
        im: 0.0,
    };

    return Rc::new(vec![vec![e, e], vec![e, -1.0 * e]]);
}
