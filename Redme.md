RandRumo
====

An implementation of the RomuDuoJr fast Prng (http://www.romu-random.org/). This is the weakest prng from the Romu
family, don't use it if the quality of randomness is paramount. However, it will most likely be good enough for
simluation or fuzzing purposes. 

Generating 10k random values compared to rusts default StdRng and faster Pcg64Mcg. At the moment it appears this
generator is about two times as fast as next best option.

```
test tests::bench_std  ... bench:      55,761 ns/iter (+/- 9,016)
test tests::bench_fast ... bench:      14,941 ns/iter (+/- 1,555)
test tests::bench_romu ... bench:       8,446 ns/iter (+/- 797)
```
