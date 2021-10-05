use num::Complex;
use std::rc::Rc;

pub type Qubit = Vec<Complex<f64>>;

pub type Gate = Rc<Matrix>;

pub type Matrix = Vec<Vec<Complex<f64>>>;

pub struct State {
    number_of_bit: u32,
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
            n = self.number_of_bit as usize,
        )
    }
}

pub struct Q {
    qb: Qubit,
}

impl Q {
    pub fn new() -> Q {
        Q { qb: vec![] }
    }

    pub fn add(&mut self, qb: Qubit) -> u32 {
        if self.qb.is_empty() {
            self.qb = qb;
            return 0;
        }

        self.tensor_vec(qb);
        self.number_of_bit() - 1
    }

    pub fn zero(&mut self) -> u32 {
        self.add(vec![
            Complex { re: 1.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
        ])
    }

    pub fn zero_with(&mut self, n: u32) -> Vec<u32> {
        let mut list: Vec<u32> = vec![];

        for _ in 0..n {
            list.push(self.zero());
        }

        list
    }

    pub fn zero_log2(&mut self, n: u32) -> Vec<u32> {
        let log2n: u32 = ((n as f64).log2() as u32) + 1;
        self.zero_with(log2n)
    }

    fn tensor_vec(&mut self, qb: Qubit) {
        let mut v: Qubit = vec![];

        for w in &self.qb {
            for j in &qb {
                v.push(w * j);
            }
        }

        self.qb = v
    }

    pub fn number_of_bit(&self) -> u32 {
        (self.qb.len() as f64).log2() as u32
    }

    pub fn x(&mut self, qb: &[u32]) {
        self.apply(x(), qb);
    }

    pub fn h(&mut self, qb: &[u32]) {
        self.apply(h(), qb);
    }

    pub fn apply(&mut self, g: Gate, qb: &[u32]) {
        let list: Vec<Gate> = gate_list(self.number_of_bit(), g, id(), qb);
        let g: Gate = tensor_(&list);
        let mut v: Qubit = vec![];

        for i in 0..g.len() {
            let mut e = Complex { re: 0.0, im: 0.0 };

            for j in 0..g[i].len() {
                e += g[i][j] * self.qb[j];
            }

            v.push(e);
        }

        self.qb = v
    }

    pub fn cmodexp2(&mut self, a: u32, n: u32, r0: &[u32], r1: &[u32]) {
        println!("cmodexp2({}, {}, {:?}, {:?})", a, n, r0, r1);
    }

    pub fn iqft(&mut self, qb: &[u32]) {
        println!("iqft({:?})", qb);
    }

    pub fn state(&self) -> Vec<State> {
        let z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
        let n: u32 = self.number_of_bit();
        let mut list: Vec<State> = vec![];

        for i in 0..self.qb.len() {
            if self.qb[i] == z {
                continue;
            }

            list.push(State {
                number_of_bit: n,
                index: i,
                amp: self.qb[i],
                prob: self.qb[i].norm().powf(2.0),
            });
        }

        list
    }
}

fn tensor_(list: &[Gate]) -> Gate {
    let mut g: Gate = Rc::clone(&list[0]);

    for i in list.iter().skip(1) {
        g = tensor(g, Rc::clone(i));
    }

    g
}

fn tensor(m: Gate, n: Gate) -> Gate {
    let mut g: Matrix = vec![];

    for i in 0..m.len() {
        for k in 0..n.len() {
            let mut v: Vec<Complex<f64>> = vec![];

            for j in 0..m[i].len() {
                for l in 0..n[k].len() {
                    v.push(m[i][j] * n[k][l]);
                }
            }

            g.push(v);
        }
    }

    Rc::new(g)
}

fn gate_list(n: u32, g: Gate, id: Gate, qb: &[u32]) -> Vec<Gate> {
    let mut list: Vec<Gate> = vec![];

    for i in 0..n {
        let mut found = false;

        for j in qb {
            if i == *j {
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

    list
}

fn id() -> Gate {
    Rc::new(vec![
        vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }],
        vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }],
    ])
}

fn x() -> Gate {
    Rc::new(vec![
        vec![Complex { re: 0.0, im: 0.0 }, Complex { re: 1.0, im: 0.0 }],
        vec![Complex { re: 1.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }],
    ])
}

fn h() -> Gate {
    let e = Complex {
        re: 1.0 / std::f64::consts::SQRT_2,
        im: 0.0,
    };

    Rc::new(vec![vec![e, e], vec![e, -1.0 * e]])
}
