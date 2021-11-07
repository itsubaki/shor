use num::Complex;
use num::Zero;
use std::rc::Rc;

pub type Complex64 = Complex<f64>;

pub type Qubit = Vec<Complex64>;

pub type Gate = Vec<Vec<Complex64>>;

pub type BinaryChars = Vec<char>;

pub struct State {
    number_of_bit: u32,
    pub index: usize,
    pub amp: Complex64,
    pub prob: f64,
}

impl State {
    pub fn to_binary_chars(&self, qb: &[u32]) -> BinaryChars {
        let v = to_binary_chars(self.index, self.number_of_bit as usize);

        let mut bin = vec![];
        for i in qb {
            bin.push(v[*i as usize]);
        }

        bin
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, dest: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            dest,
            "{:>0n$b} {:>+.4} {:>+.4} {:>.4}",
            self.index,
            self.amp.re,
            self.amp.im,
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

        self.tensor(qb);
        self.number_of_bit() - 1
    }

    pub fn zero(&mut self) -> u32 {
        self.add(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)])
    }

    pub fn zero_with(&mut self, n: u32) -> Vec<u32> {
        let mut list = vec![];

        for _ in 0..n {
            list.push(self.zero());
        }

        list
    }

    pub fn zero_log2(&mut self, n: u32) -> Vec<u32> {
        let log2n = ((n as f64).log2() as u32) + 1;
        self.zero_with(log2n)
    }

    fn tensor(&mut self, qb: Qubit) {
        let mut v = vec![];

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
        self.apply_with(x(), qb)
    }

    pub fn h(&mut self, qb: &[u32]) {
        self.apply_with(h(), qb)
    }

    fn apply_with(&mut self, g: Gate, qb: &[u32]) {
        let list = gate_list(self.number_of_bit(), g, qb);
        let g = tensor_with(&list);
        self.apply(g)
    }

    #[allow(clippy::needless_range_loop)]
    pub fn apply(&mut self, g: Gate) {
        let mut v = vec![];

        for i in 0..g.len() {
            let mut e = Complex::new(0.0, 0.0);

            for j in 0..g[i].len() {
                e += g[i][j] * self.qb[j];
            }

            v.push(e);
        }

        self.qb = v
    }

    pub fn cmodexp2(&mut self, a: u32, n: u32, r0: &[u32], r1: &[u32]) {
        let nob = self.number_of_bit();
        for (i, c) in r0.iter().enumerate() {
            self.apply(cmodexp2(nob, a, i as u32, n, *c, r1))
        }
    }

    pub fn iqft(&mut self, qb: &[u32]) {
        let len = qb.len();

        // for i := l - 1; i > -1; i-- {}
        for i in (0..len).rev() {
            let mut k = (len - i) as i32;

            // for j := l - 1; j > i; j-- {}
            for j in ((i + 1)..len).rev() {
                self.icr(k, qb[j], qb[i]);
                k -= 1;
            }

            self.h(&[qb[i]]);
        }
    }

    pub fn icr(&mut self, k: i32, control: u32, target: u32) {
        let n = self.number_of_bit();
        let g = dagger(cr(k, n, control, target));
        self.apply(g)
    }

    pub fn state(&self) -> Vec<State> {
        let mut list = vec![];
        let nob = self.number_of_bit();

        for i in 0..self.qb.len() {
            let rqb = round(self.qb[i]);
            if rqb.is_zero() {
                continue;
            }

            list.push(State {
                number_of_bit: nob,
                index: i,
                amp: rqb,
                prob: rqb.norm().powf(2.0),
            });
        }

        list
    }
}

fn tensor_with(list: &[Rc<Gate>]) -> Gate {
    let mut g = list[0].to_vec();

    for i in list.iter().skip(1) {
        g = tensor(g, Rc::clone(i));
    }

    g
}

#[allow(clippy::needless_range_loop)]
fn tensor(m: Gate, n: Rc<Gate>) -> Gate {
    let mut g = vec![];

    for i in 0..m.len() {
        for k in 0..n.len() {
            let mut v = vec![];

            for j in 0..m[i].len() {
                for l in 0..n[k].len() {
                    v.push(m[i][j] * n[k][l]);
                }
            }

            g.push(v);
        }
    }

    g
}

fn gate_list(nob: u32, g: Gate, qb: &[u32]) -> Vec<Rc<Gate>> {
    let mut list = vec![];
    let identity = Rc::new(id());
    let rg = Rc::new(g);

    for i in 0..nob {
        let mut found = false;

        for j in qb {
            if i == *j {
                found = true;
                break;
            }
        }

        if found {
            list.push(Rc::clone(&rg));
            continue;
        }

        list.push(Rc::clone(&identity));
    }

    list
}

fn id() -> Gate {
    id_with(1)
}

fn x() -> Gate {
    vec![
        vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ]
}

fn h() -> Gate {
    let e = Complex::new(1.0 / std::f64::consts::SQRT_2, 0.0);
    vec![vec![e, e], vec![e, -1.0 * e]]
}

fn cr(k: i32, nob: u32, control: u32, target: u32) -> Gate {
    // identity matrix
    let mut g = id_with(nob);

    // coefficient
    let p = 2.0 * std::f64::consts::PI / (2.0_f64.powf(k as f64));
    let e = Complex::new(0.0, p).exp();

    for (i, v) in g.iter_mut().enumerate() {
        let bits = to_binary_chars(i, nob as usize);
        if bits[control as usize] == '1' && bits[target as usize] == '1' {
            // apply
            v[i] = e * v[i];
        }
    }

    transpose(g)
}

fn cmodexp2(nob: u32, a: u32, j: u32, n: u32, control: u32, target: &[u32]) -> Gate {
    let r0len = nob - target.len() as u32;
    let r1len = target.len() as u32;
    let a2jmodn = super::number::modexp2(a, j, n);

    let mut index = vec![];
    for i in 0..(2_usize.pow(nob)) {
        let bits = to_binary_chars(i, nob as usize);
        if bits[control as usize] == '0' {
            // i -> i
            index.push(to_decimal(&bits) as usize);
            continue;
        }

        let r1bits = take(&bits, r0len as usize, bits.len());
        let k = to_decimal(&r1bits);
        if k > n - 1 {
            // i -> i
            index.push(to_decimal(&bits) as usize);
            continue;
        }

        // i -> a**2**j *k mod n
        let a2jkmodn = (a2jmodn * k) % n;
        let mut a2jkmodns = to_binary_chars(a2jkmodn as usize, r1len as usize);

        let mut r0bits = take(&bits, 0, r0len as usize);
        r0bits.append(&mut a2jkmodns);
        index.push(to_decimal(&r0bits) as usize);
    }

    let identity = id_with(nob);
    let mut g = vec![vec![]; identity.len()];
    for (i, ii) in index.iter().enumerate() {
        g[i] = clone_vec(&identity[*ii]);
    }

    transpose(g)
}

fn round(c: Complex64) -> Complex64 {
    let mut round = c;
    if c.re.abs() < 1e-13 {
        round.re = 0.0;
    }

    if c.im.abs() < 1e-13 {
        round.im = 0.0;
    }

    round
}

fn take(bin: &[char], start: usize, end: usize) -> BinaryChars {
    bin[start..end].to_vec()
}

fn to_binary_chars(i: usize, nob: usize) -> BinaryChars {
    format!("{:>0n$b}", i, n = nob).chars().collect()
}

fn to_decimal(v: &[char]) -> u32 {
    let s: String = v.iter().collect();
    u32::from_str_radix(&s, 2).unwrap()
}

fn id_with(nob: u32) -> Vec<Vec<Complex64>> {
    let mut mat = vec![];

    for i in 0..(2_i32.pow(nob)) {
        let mut v = vec![];

        for j in 0..(2_i32.pow(nob)) {
            if i == j {
                v.push(Complex::new(1.0, 0.0));
                continue;
            }

            v.push(Complex::new(0.0, 0.0));
        }

        mat.push(v);
    }

    mat
}

fn clone_vec(v: &[Complex64]) -> Vec<Complex64> {
    let mut clone = vec![];

    for i in v {
        clone.push(*i);
    }

    clone
}

fn dagger(g: Gate) -> Gate {
    transpose(conjugate(g))
}

#[allow(clippy::needless_range_loop)]
fn transpose(g: Gate) -> Gate {
    let mut trans = vec![];

    for i in 0..g.len() {
        let mut v = vec![];

        for j in 0..g[i].len() {
            v.push(g[j][i])
        }

        trans.push(v);
    }

    trans
}

#[allow(clippy::needless_range_loop)]
fn conjugate(g: Gate) -> Gate {
    let mut conj = vec![];

    for i in 0..g.len() {
        let mut v = vec![];

        for j in 0..g[i].len() {
            v.push(g[i][j].conj());
        }

        conj.push(v);
    }

    conj
}

#[test]
fn test_to_binary_chars() {
    assert_eq!(to_binary_chars(3, 5), vec!['0', '0', '0', '1', '1']);
    assert_eq!(to_binary_chars(7, 5), vec!['0', '0', '1', '1', '1']);
    assert_eq!(to_binary_chars(15, 5), vec!['0', '1', '1', '1', '1']);
    assert_eq!(to_binary_chars(31, 5), vec!['1', '1', '1', '1', '1']);
}

#[test]
fn test_to_decimal() {
    assert_eq!(to_decimal(&['1']), 1);
    assert_eq!(to_decimal(&['1', '1']), 3);
    assert_eq!(to_decimal(&['1', '0', '1']), 5);
}

#[test]
fn test_is_eigen_vector() {
    let n = 15;
    let a = 7;
    let t = 3;

    let mut qsim = Q::new();
    let r0 = qsim.zero_with(t);
    let r1 = qsim.zero_log2(n);

    qsim.x(&[r1[r1.len() - 1]]);
    qsim.h(&r0);
    qsim.cmodexp2(a, n, &r0, &r1);
    qsim.iqft(&r0);

    let mut us = std::collections::HashMap::new();
    for state in qsim.state().iter() {
        let m1 = state.to_binary_chars(&r1);
        let ui: String = m1.iter().collect();

        let v = match us.get(&ui) {
            Some(vv) => state.amp + vv,
            None => state.amp,
        };

        us.insert(ui, v);
    }

    let cases = vec![
        ("0001", Complex::new(1.0, 0.0)),
        ("0100", Complex::new(0.0, 0.0)),
        ("0111", Complex::new(0.0, 0.0)),
        ("1101", Complex::new(0.0, 0.0)),
    ];

    for (i, c) in cases {
        let v = match us.get(i) {
            Some(v) => *v,
            None => panic!("{} not found", i),
        };

        assert!((v - c).re.abs() < 1e-13);
        assert!((v - c).im.abs() < 1e-13);
    }
}
