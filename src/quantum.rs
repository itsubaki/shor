use num::Complex;
use std::rc::Rc;

pub type Qubit = Vec<Complex<f64>>;

pub type Gate = Rc<Matrix>;

pub type Matrix = Vec<Vec<Complex<f64>>>;

pub type BinaryChars = Vec<char>;

pub struct State {
    number_of_bit: u32,
    pub index: usize,
    pub amp: Complex<f64>,
    pub prob: f64,
}

impl State {
    pub fn to_binary_chars(&self, qb: &[u32]) -> BinaryChars {
        let v: Vec<char> = to_binary_chars(self.index, self.number_of_bit as usize);

        let mut out: Vec<char> = vec![];
        for i in qb {
            out.push(v[*i as usize]);
        }

        out
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

        self.tensor_vec(qb);
        self.number_of_bit() - 1
    }

    pub fn zero(&mut self) -> u32 {
        self.add(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)])
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
        self.apply_(x(), qb)
    }

    pub fn h(&mut self, qb: &[u32]) {
        self.apply_(h(), qb)
    }

    fn apply_(&mut self, g: Gate, qb: &[u32]) {
        let list: Vec<Gate> = gate_list(self.number_of_bit(), g, qb);
        let g: Gate = tensor_(&list);
        self.apply(g)
    }

    pub fn apply(&mut self, g: Gate) {
        let mut v: Qubit = vec![];

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
        let nob: u32 = self.number_of_bit();
        for (i, c) in r0.iter().enumerate() {
            self.apply(cmodexp2(nob, a, i as u32, n, *c, r1))
        }
    }

    pub fn iqft(&mut self, qb: &[u32]) {
        let l: usize = qb.len();

        // for i := l - 1; i > -1; i-- {}
        for i in (0..l).rev() {
            let mut k: i32 = (l as i32) - (i as i32);

            // for j := l - 1; j > i; j-- {}
            for j in ((i + 1)..l).rev() {
                self.icr(k, qb[j], qb[i]);
                k -= 1;
            }

            self.h(&[qb[i]]);
        }
    }

    pub fn icr(&mut self, k: i32, control: u32, target: u32) {
        let n: u32 = self.number_of_bit();
        let g: Gate = dagger(cr(k, n, control, target));
        self.apply(g)
    }

    pub fn state(&self) -> Vec<State> {
        let z: Complex<f64> = Complex::new(0.0, 0.0);
        let nob: u32 = self.number_of_bit();
        let mut list: Vec<State> = vec![];

        for i in 0..self.qb.len() {
            let rqb: Complex<f64> = round(self.qb[i]);
            if rqb == z {
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

fn tensor_(list: &[Gate]) -> Gate {
    let mut g: Gate = Rc::clone(&list[0]);

    for i in list.iter().skip(1) {
        g = tensor(g, Rc::clone(i));
    }

    g
}

fn tensor(m: Gate, n: Gate) -> Gate {
    let mut mat: Matrix = vec![];

    for i in 0..m.len() {
        for k in 0..n.len() {
            let mut v: Vec<Complex<f64>> = vec![];

            for j in 0..m[i].len() {
                for l in 0..n[k].len() {
                    v.push(m[i][j] * n[k][l]);
                }
            }

            mat.push(v);
        }
    }

    Rc::new(mat)
}

fn gate_list(nob: u32, g: Gate, qb: &[u32]) -> Vec<Gate> {
    let identity: Gate = id();
    let mut list: Vec<Gate> = vec![];

    for i in 0..nob {
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

        list.push(Rc::clone(&identity));
    }

    list
}

fn id() -> Gate {
    Rc::new(vec![
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
    ])
}

fn x() -> Gate {
    Rc::new(vec![
        vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
    ])
}

fn h() -> Gate {
    let e = Complex::new(1.0 / std::f64::consts::SQRT_2, 0.0);
    Rc::new(vec![vec![e, e], vec![e, -1.0 * e]])
}

fn cr(k: i32, nob: u32, control: u32, target: u32) -> Gate {
    // identity matrix
    let mut mat: Matrix = idmat(nob);

    // coefficient
    let p = 2.0 * std::f64::consts::PI / (2.0_f64.powf(k as f64));
    let e = Complex::new(0.0, p).exp();

    for (i, v) in mat.iter_mut().enumerate() {
        let bits: BinaryChars = to_binary_chars(i, nob as usize);
        if bits[control as usize] == '1' && bits[target as usize] == '1' {
            // apply
            v[i] = e * v[i];
        }
    }

    transpose(Rc::new(mat))
}

fn cmodexp2(nob: u32, a: u32, j: u32, n: u32, control: u32, target: &[u32]) -> Gate {
    let r0len: u32 = nob - target.len() as u32;
    let r1len: u32 = target.len() as u32;
    let a2jmodn = super::number::modexp2(a, j, n);

    let mut index: Vec<usize> = vec![];
    for i in 0..(2_usize.pow(nob)) {
        let bits: BinaryChars = to_binary_chars(i, nob as usize);
        if bits[control as usize] == '0' {
            // i -> i
            index.push(to_decimal(&bits));
            continue;
        }

        let r1bits: BinaryChars = take(&bits, r0len as usize, bits.len());
        let k: usize = to_decimal(&r1bits);
        if (k as u32) > n - 1 {
            // i -> i
            index.push(to_decimal(&bits));
            continue;
        }

        // i -> a**2**j *k mod n
        let a2jkmodn: u32 = (a2jmodn * k as u32) % n;
        let mut a2jkmodns: BinaryChars = to_binary_chars(a2jkmodn as usize, r1len as usize);

        let mut r0bits: BinaryChars = take(&bits, 0, r0len as usize);
        r0bits.append(&mut a2jkmodns);
        index.push(to_decimal(&r0bits));
    }

    let mat: Matrix = idmat(nob);
    let mut out: Matrix = vec![vec![]; mat.len()];
    for (i, ii) in index.iter().enumerate() {
        out[i] = clone(&mat[*ii]); // :(
    }

    transpose(Rc::new(out))
}

fn round(c: Complex<f64>) -> Complex<f64> {
    let mut out: Complex<f64> = c;
    if c.re.abs() < 1e-13 {
        out.re = 0.0;
    }

    if c.im.abs() < 1e-13 {
        out.im = 0.0;
    }

    out
}

fn take(bin: &[char], start: usize, end: usize) -> BinaryChars {
    bin[start..end].to_vec()
}

fn to_binary_chars(i: usize, nob: usize) -> BinaryChars {
    format!("{:>0n$b}", i, n = nob).chars().collect()
}

fn to_decimal(v: &[char]) -> usize {
    let s: String = v.iter().collect();
    usize::from_str_radix(&s, 2).unwrap()
}

fn idmat(nob: u32) -> Matrix {
    let mut mat: Matrix = vec![];

    for i in 0..(2_i32.pow(nob)) {
        let mut v: Vec<Complex<f64>> = vec![];

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

fn clone(v: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let mut out: Vec<Complex<f64>> = vec![];
    for i in v {
        out.push(*i);
    }

    out
}

fn dagger(g: Gate) -> Gate {
    transpose(conjugate(g))
}

fn transpose(g: Gate) -> Gate {
    let mut mat: Matrix = vec![];

    for i in 0..g.len() {
        let mut v: Vec<Complex<f64>> = vec![];

        for j in 0..g[i].len() {
            v.push(g[j][i])
        }

        mat.push(v);
    }

    Rc::new(mat)
}

fn conjugate(g: Gate) -> Gate {
    let mut mat: Matrix = vec![];

    for i in 0..g.len() {
        let mut v: Vec<Complex<f64>> = vec![];

        for j in 0..g[i].len() {
            v.push(g[i][j].conj());
        }

        mat.push(v);
    }

    Rc::new(mat)
}

#[test]
fn test_is_eigen_vector() {
    let n: u32 = 15;
    let a: u32 = 7;
    let t: u32 = 3;

    let mut qsim = Q::new();
    let r0 = qsim.zero_with(t);
    let r1 = qsim.zero_log2(n);

    qsim.x(&[r1[r1.len() - 1]]);
    qsim.h(&r0);
    qsim.cmodexp2(a, n, &r0, &r1);
    qsim.iqft(&r0);

    let mut us: std::collections::HashMap<String, Complex<f64>> = std::collections::HashMap::new();
    for state in qsim.state().iter() {
        let m1: Vec<char> = state.to_binary_chars(&r1);
        let ui: String = m1.iter().collect();

        let v: Complex<f64> = match us.get(&ui) {
            Some(vv) => state.amp + vv,
            None => state.amp,
        };

        us.insert(ui, v);
    }

    for (m, v) in &us {
        println!("{:?} {}", m, v);
    }

    let v: Complex<f64> = match us.get("0001") {
        Some(v) => *v,
        None => panic!("0001 not found"),
    };

    assert!((v.re - 1.0) < 1e-13);
    assert!((v.im - 0.0) < 1e-13);
}
