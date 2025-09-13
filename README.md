# rust-thin-lto-perf-degradation

This is a reproducer for a surprising performance regression of a code
compiled with `-C target-cpu=x86-64-v3` (or `-C target-cpu=native`) and
`"fat"` LTO enabled. This reproducer require `codegen-units = 32` to
demonstrate the issue, but I originally encountered this issue with the
default value of `codegen-units` (which is `16`) while working on a
benchmarks within [nalgebra](https://github.com/sebcrozet/nalgebra)
crate.

## usage

```bash
./run
```

## Results

More than **4000%** performance degradation between `-C target-cpu=x86-64-v2`
and `-C target-cpu=x86-64-v3` with `"fat"` LTO and `codegen-units = 32`
was successfully reproduced with

```text
rustc 1.89.0 (29483883e 2025-08-04) (Fedora 1.89.0-2.fc42)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: x86_64-unknown-linux-gnu
release: 1.89.0
LLVM version: 20.1.8

and

rustc 1.89.0 (29483883e 2025-08-04)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: x86_64-unknown-linux-gnu
release: 1.89.0
LLVM version: 20.1.7
```

on AMD Ryzen 9 5950X and AMD Ryzen 9 9950X.

Benchmarking result looks like this:

```text
bench_mat4_transform_point3
                        time:   [18.883 ns 18.986 ns 19.093 ns]
                        change: [+4284.5% +4303.0% +4320.2%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
```

Tested combinations:

| Profile | LTO | CU | Perf diff |
|---------|-----|----:|----------:|
| no-lto | `false` | - | -3.3% |
| thin-local-lto-32cu | `"off"` | 32 | +2.77% |
| fat-lto-1cu | `"fat"` | 1 | +1.27% |
| fat-lto-32cu | `"fat"` | 32 | +6722.8% |
| thin-lto-1cu | `"thin"` | 1 | +5.18% |
| thin-lto-32cu | `"thin"` | 32 | +5.52% |

("Perf diff" show difference between `-C target-cpu=x86-64-v2`
and `-C target-cpu=x86-64-v3` on AMD Ryzen 9 9950X)

## `target-cpu` vs. `tune-cpu`

Issue is not reproducible with

```bash
RUSTFLAGS="-C target-cpu=x86-64-v2 -C target-feature=+avx,+avx2,+bmi1,+bmi2,+f16c,+fma,+lzcnt,+movbe,+xsave"
```

And reproducible with

```bash
RUSTFLAGS="-C target-cpu=x86-64-v3 -C target-feature=-avx,-avx2,-bmi1,-bmi2,-f16c,-fma,-lzcnt,-movbe,-xsave" cargo run --quiet --profile "fat-lto-32cu" -- --bench --baseline baseline
```

So I suppose that this has something to do with the `tune-cpu` and not with actually enabled target features.

## Detailed output

(on AMD Ryzen 9 9950X)

```text
                    *** With profile no-lto ***

+ RUSTFLAGS=-C target-cpu=x86-64-v2 cargo run --quiet --profile no-lto -- --bench --save-baseline baseline
+ RUSTFLAGS=-C target-cpu=x86-64-v3 cargo run --quiet --profile no-lto -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
                        time:   [179.68 ps 179.69 ps 179.71 ps]
                        change: [−3.3208% −3.3061% −3.2924%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild


                    *** With profile thin-local-lto-32cu ***

+ RUSTFLAGS=-C target-cpu=x86-64-v2 cargo run --quiet --profile thin-local-lto-32cu -- --bench --save-baseline baseline
+ RUSTFLAGS=-C target-cpu=x86-64-v3 cargo run --quiet --profile thin-local-lto-32cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
                        time:   [187.05 ps 187.09 ps 187.12 ps]
                        change: [+2.7430% +2.7704% +2.7975%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe


                    *** With profile fat-lto-1cu ***

+ RUSTFLAGS=-C target-cpu=x86-64-v2 cargo run --quiet --profile fat-lto-1cu -- --bench --save-baseline baseline
+ RUSTFLAGS=-C target-cpu=x86-64-v3 cargo run --quiet --profile fat-lto-1cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
                        time:   [188.64 ps 188.65 ps 188.66 ps]
                        change: [+1.2596% +1.2725% +1.2845%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe


                    *** With profile fat-lto-32cu ***

+ RUSTFLAGS=-C target-cpu=x86-64-v2 cargo run --quiet --profile fat-lto-32cu -- --bench --save-baseline baseline
+ RUSTFLAGS=-C target-cpu=x86-64-v3 cargo run --quiet --profile fat-lto-32cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
                        time:   [12.927 ns 12.927 ns 12.928 ns]
                        change: [+6721.9% +6722.8% +6723.9%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe


                    *** With profile thin-lto-1cu ***

+ RUSTFLAGS=-C target-cpu=x86-64-v2 cargo run --quiet --profile thin-lto-1cu -- --bench --save-baseline baseline
+ RUSTFLAGS=-C target-cpu=x86-64-v3 cargo run --quiet --profile thin-lto-1cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
                        time:   [189.07 ps 189.08 ps 189.10 ps]
                        change: [+5.1711% +5.1866% +5.2019%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild


                    *** With profile thin-lto-32cu ***

+ RUSTFLAGS=-C target-cpu=x86-64-v2 cargo run --quiet --profile thin-lto-32cu -- --bench --save-baseline baseline
+ RUSTFLAGS=-C target-cpu=x86-64-v3 cargo run --quiet --profile thin-lto-32cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
                        time:   [189.73 ps 189.75 ps 189.77 ps]
                        change: [+5.5056% +5.5284% +5.5503%] (p = 0.00 < 0.05)
                        Performance has regressed.
```
