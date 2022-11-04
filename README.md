## Benchmark between ahashMap and ahash with rust-intmap

I would like to have the lowest lookup time for a short input string key.
I found [ahash](https://github.com/tkaitchuck/ahash) when it has short string key (faster than xxhash3!)
![](https://camo.githubusercontent.com/64061a97fc30ee4f5120c5d7d94d80a23501d4045f3fb21322ea7aedfa0856fa/68747470733a2f2f646f63732e676f6f676c652e636f6d2f7370726561647368656574732f642f652f32504143582d3176534b374c69326e532d4275723961724159463949665433374d502d6f684165317631396c5a75356664394d616a4931665376654c41515a79456965344561396b352d535748546666376e4c3244572f70756263686172743f6f69643d3133323336313839333826666f726d61743d696d616765)

And I found this interesting project, [rust-intmap](https://github.com/JesperAxelsson/rust-intmap). Where it used a specialised hash function for u64 type, utilizing cpu internal cache to avoid expensive modulus operator.

Therefore I made an experiment to try out the combination of ahash and intmap. Here is the result of it.

```sh
cargo bench
```

Tested on Mac mini M1, 2020, 16GB Memory

```
    Finished bench [optimized] target(s) in 0.05s
     Running benches/build.rs (target/release/deps/build-fe5693c1400d7f24)
lookup ahash-intmap     time:   [656.19 µs 656.63 µs 657.12 µs]
                        change: [-0.8081% -0.4065% -0.0520%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

lookup hashmap          time:   [825.73 µs 828.95 µs 833.43 µs]
                        change: [-1.9067% -0.5120% +0.3923%] (p = 0.50 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

lookup ahashmap         time:   [668.34 µs 669.00 µs 669.69 µs]
                        change: [-0.4299% -0.0904% +0.2378%] (p = 0.61 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
  6 (6.00%) high severe
```