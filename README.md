# rust-fat-lto-perf-degradation

This is a reproducer for a surprising performance regression of a code
compiled with `-C target-cpu=x86-64-v3` (or `-C target-cpu=native`) and
`"fat"` LTO enabled. This reproducer require `codegen-units = 32` to
demonstrate the issue, but I originally encountered this issue with the
default value of `codegen-units` (which is `16`) while working on a
benchmarks within [nalgebra](https://github.com/sebcrozet/nalgebra)
crate.

## Usage

**IMPORTANT:** You need a CPU which supports the `x86-64-v3` feature set.

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

On AMD Ryzen 9 9950X: <details><summary>click for details...</summary>

```text
rustc 1.89.0 (29483883e 2025-08-04)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: x86_64-unknown-linux-gnu
release: 1.89.0
LLVM version: 20.1.7

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
</details>

On AMD Ryzen 9 5950X: <details><summary>click for details...</summary>

```text
rustc 1.89.0 (29483883e 2025-08-04) (Fedora 1.89.0-2.fc42)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: x86_64-unknown-linux-gnu
release: 1.89.0
LLVM version: 20.1.8

                    *** With profile no-lto ***

+ RUSTFLAGS='-C target-cpu=x86-64-v2'
+ cargo run --quiet --profile no-lto -- --bench --save-baseline baseline
+ RUSTFLAGS='-C target-cpu=x86-64-v3'
+ cargo run --quiet --profile no-lto -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
                        time:   [424.12 ps 424.45 ps 424.74 ps]
                        change: [−0.0075% +0.1427% +0.2726%] (p = 0.05 > 0.05)
                        No change in performance detected.


                    *** With profile thin-local-lto-32cu ***

+ RUSTFLAGS='-C target-cpu=x86-64-v2'
+ cargo run --quiet --profile thin-local-lto-32cu -- --bench --save-baseline baseline
+ RUSTFLAGS='-C target-cpu=x86-64-v3'
+ cargo run --quiet --profile thin-local-lto-32cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
                        time:   [428.58 ps 429.11 ps 429.83 ps]
                        change: [+1.6072% +1.8958% +2.2862%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe


                    *** With profile fat-lto-1cu ***

+ RUSTFLAGS='-C target-cpu=x86-64-v2'
+ cargo run --quiet --profile fat-lto-1cu -- --bench --save-baseline baseline
+ RUSTFLAGS='-C target-cpu=x86-64-v3'
+ cargo run --quiet --profile fat-lto-1cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
                        time:   [426.12 ps 426.28 ps 426.51 ps]
                        change: [+0.0483% +0.1500% +0.2519%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) high mild
  12 (12.00%) high severe


                    *** With profile fat-lto-32cu ***

+ RUSTFLAGS='-C target-cpu=x86-64-v2'
+ cargo run --quiet --profile fat-lto-32cu -- --bench --save-baseline baseline
+ RUSTFLAGS='-C target-cpu=x86-64-v3'
+ cargo run --quiet --profile fat-lto-32cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
                        time:   [18.943 ns 18.960 ns 18.985 ns]
                        change: [+4369.3% +4375.3% +4381.7%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 15 outliers among 100 measurements (15.00%)
  7 (7.00%) high mild
  8 (8.00%) high severe


                    *** With profile thin-lto-1cu ***

+ RUSTFLAGS='-C target-cpu=x86-64-v2'
+ cargo run --quiet --profile thin-lto-1cu -- --bench --save-baseline baseline
+ RUSTFLAGS='-C target-cpu=x86-64-v3'
+ cargo run --quiet --profile thin-lto-1cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
                        time:   [426.01 ps 426.45 ps 427.04 ps]
                        change: [+0.3322% +0.5013% +0.6534%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  9 (9.00%) high severe


                    *** With profile thin-lto-32cu ***

+ RUSTFLAGS='-C target-cpu=x86-64-v2'
+ cargo run --quiet --profile thin-lto-32cu -- --bench --save-baseline baseline
+ RUSTFLAGS='-C target-cpu=x86-64-v3'
+ cargo run --quiet --profile thin-lto-32cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
                        time:   [424.16 ps 424.47 ps 424.86 ps]
                        change: [−0.7177% −0.6390% −0.5660%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe
```
</details>

On Intel Core i7-8565U: <details><summary>click for details...</summary>

```text
rustc 1.89.0 (29483883e 2025-08-04) (Fedora 1.89.0-2.fc42)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: x86_64-unknown-linux-gnu
release: 1.89.0
LLVM version: 20.1.8

*** With profile no-lto ***

+ RUSTFLAGS='-C target-cpu=x86-64-v2'
+ cargo run --quiet --profile no-lto -- --bench --save-baseline baseline
+ RUSTFLAGS='-C target-cpu=x86-64-v3'
+ cargo run --quiet --profile no-lto -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
    time:   [489.43 ps 491.40 ps 494.23 ps]
    change: [+1.9938% +2.6994% +3.7127%] (p = 0.00 < 0.05)
    Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
2 (2.00%) high mild
6 (6.00%) high severe


*** With profile thin-local-lto-32cu ***

+ RUSTFLAGS='-C target-cpu=x86-64-v2'
+ cargo run --quiet --profile thin-local-lto-32cu -- --bench --save-baseline baseline
+ RUSTFLAGS='-C target-cpu=x86-64-v3'
+ cargo run --quiet --profile thin-local-lto-32cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
    time:   [554.53 ps 564.05 ps 573.35 ps]
    change: [+0.0623% +1.0707% +1.9903%] (p = 0.03 < 0.05)
    Change within noise threshold.
Found 19 outliers among 100 measurements (19.00%)
1 (1.00%) low mild
1 (1.00%) high mild
17 (17.00%) high severe


*** With profile fat-lto-1cu ***

+ RUSTFLAGS='-C target-cpu=x86-64-v2'
+ cargo run --quiet --profile fat-lto-1cu -- --bench --save-baseline baseline
+ RUSTFLAGS='-C target-cpu=x86-64-v3'
+ cargo run --quiet --profile fat-lto-1cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
    time:   [524.70 ps 525.18 ps 525.72 ps]
    change: [+0.2331% +0.4071% +0.5726%] (p = 0.00 < 0.05)
    Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
2 (2.00%) high severe


*** With profile fat-lto-32cu ***

+ RUSTFLAGS='-C target-cpu=x86-64-v2'
+ cargo run --quiet --profile fat-lto-32cu -- --bench --save-baseline baseline
+ RUSTFLAGS='-C target-cpu=x86-64-v3'
+ cargo run --quiet --profile fat-lto-32cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
    time:   [17.077 ns 17.167 ns 17.311 ns]
    change: [+3443.6% +3455.3% +3471.8%] (p = 0.00 < 0.05)
    Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
2 (2.00%) low mild
2 (2.00%) high mild
7 (7.00%) high severe


*** With profile thin-lto-1cu ***

+ RUSTFLAGS='-C target-cpu=x86-64-v2'
+ cargo run --quiet --profile thin-lto-1cu -- --bench --save-baseline baseline
+ RUSTFLAGS='-C target-cpu=x86-64-v3'
+ cargo run --quiet --profile thin-lto-1cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
    time:   [577.63 ps 579.07 ps 581.24 ps]
    change: [+3.9613% +6.2773% +8.3700%] (p = 0.00 < 0.05)
    Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
1 (1.00%) high mild
1 (1.00%) high severe


*** With profile thin-lto-32cu ***

+ RUSTFLAGS='-C target-cpu=x86-64-v2'
+ cargo run --quiet --profile thin-lto-32cu -- --bench --save-baseline baseline
+ RUSTFLAGS='-C target-cpu=x86-64-v3'
+ cargo run --quiet --profile thin-lto-32cu -- --bench --baseline baseline
Gnuplot not found, using plotters backend
bench_mat4_transform_point3
    time:   [501.14 ps 503.39 ps 506.85 ps]
    change: [−13.570% −12.560% −11.725%] (p = 0.00 < 0.05)
    Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
1 (1.00%) low mild
4 (4.00%) high mild
5 (5.00%) high severe
```
</details>
