## cram
> A one stop shop for (un)archiving and (de)compressing files.
## Benchmarking
Full details can be found in [`benchmark.json`](benchmark.json).

Benchmarks are computed using the `cram` and `tar` binaries, unoptimized. The same compression binaries are used for both, to ensure a fair comparison (the Rust libraries are significantly less effiecient).

But here are some summary graphs:

![1KB random file](benchmark/1KB_RANDOM.png)
![1MB random file](benchmark/1MB_RANDOM.png)
![10MB random file](benchmark/10MB_RANDOM.png)
![werdl/mintext](benchmark/mintext.png)
![werdl/os](benchmark/os.png)
![werdl/xz](benchmark/xz.png)