# Line per line : (3fc2378)

## Debug

```bash
real 7m0.214s
user 6m56.554s
sys 0m2.510s
```

## Release

```bash
real    1m19.206s
user    1m17.445s
sys     0m1.884s
```

# 512 bytes buffers : (d1f525f)

## Debug

```bash
real    7m33.604s
user    7m31.431s
sys     0m2.156s
```

## Release

```bash
real    1m11.068s
user    1m9.229s
sys     0m1.985s
```

No difference between the versions.
Either the read_line is already optimized or I did something that hampers this second version.
