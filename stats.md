# Line per line : (3fc2378)

```bash
time cargo run --release measurements_e9.txt
```

```bash
real    1m19.206s
user    1m17.445s
sys     0m1.884s
```

# 512 bytes buffers : (d1f525f)

```bash
time cargo run --release measurements_e9.txt
```

```bash
real    1m11.068s
user    1m9.229s
sys     0m1.985s
```

No difference between the versions.
Either the read_line is already optimized or I did something that hampers this second version.

Edit: After looking into BufReader, it already reads large blocks of data and keeps it in memory.

# Split process on multiple threads : (b09dd31)

```bash
time cargo run --release measurements_e9.txt 4
```

```bash
real    0m20.001s
user    1m17.624s
sys     0m1.409s
```

```bash
time cargo run --release measurements_e9.txt 8
```

```bash
real    0m11.774s
user    1m30.297s
sys     0m1.540s
```

```bash
time cargo run --release measurements_e9.txt 16
```

```bash
real    0m7.789s
user    2m0.183s
sys     0m1.858s
```
