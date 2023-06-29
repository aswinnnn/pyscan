# 🚀 Benchmarks

- performed on a `requirements.txt` containing **234** packages along with their versions.
- I know a benchmark is usually done with something to compare against, but I couldn't find anything like pyscan, at least not yet.
- Reccomend something that can be tested along with pyscan!
- the benchmark has been performed, using [hyperfine](https://github.com/sharkdp/hyperfine) with the following command :

```bash
hyperfine --runs 3 '.\target\release\pyscan.exe' --shell=none --export-markdown benchmarks.md --warmup 1
```

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `'.\target\release\pyscan.exe'` | 23.345 ± 0.892 | 22.731 | 24.369 | 1.00 |


- As pyscan mainly depends on making API calls, this benchmark is obviously almost variable.

There will be consistent effort regarding the optimization of pyscan in the future. This benchmark was 6min 8s before I switched to a batched API and started using references instead of moving, imagine what it'll be in the coming months! Still learning, still growing.