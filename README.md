# shor

Shor's factoring algorithm written in Rust

```
cargo install shor
```

```
$ shor 15
N: 15 (a: 8, t: 3)
[0100001](+0.2500 +0.0000): 0.0625; s/r= 1/ 4; p=3, q=5
[1100001](+0.2500 +0.0000): 0.0625; s/r= 3/ 4; p=3, q=5
```

```
$ shor 85
N: 85 (a: 2, t: 3)
[0010000001](+0.1250 +0.0000): 0.0156; s/r= 1/ 8; p=5, q=17
[0110000001](+0.1250 +0.0000): 0.0156; s/r= 3/ 8; p=5, q=17
[1010000001](+0.1250 +0.0000): 0.0156; s/r= 5/ 8; p=5, q=17
[1110000001](+0.1250 +0.0000): 0.0156; s/r= 7/ 8; p=5, q=17
```

```
$ shor 21 5
N: 21 (a: 11, t: 5)
[0010100001](+0.0822 +0.1230): 0.0219; s/r= 1/ 6; p=7, q=3
[1101100001](+0.0822 -0.1230): 0.0219; s/r= 5/ 6; p=7, q=3
```
