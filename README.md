dc-rs
=====

```
dc-rs v0.1.0
Copyright (c) 2023 Syed Fasiuddin
Report bugs at: https://github.com/SyedFasiuddin/dc-rs

This is free software with ABSOLUTELY NO WARRANTY.
usage: dc-rs [options]

dc is a reverse-polish notation command-line calculator which supports unlimited
precision arithmetic. For details, use `man dc` or see the online documentation
at https://git.yzena.com/gavin/bc/src/tag/4.0.2/manuals/bc/BUILD_TYPE.1.md.

dc-rs is a variation of dc written in Rust.
This version does not try to have one to one parity with every feature of dc(1).
One most important variation is the scale, the original dc provides arbitrary
precision calculation where as this version is limited by the limits that Rust
has for its f64 floating point type i.e. 1.7976931348623157E+308f64 (max) and
-1.7976931348623157E+308f64 (min)
```

Others:

- [dc4](https://github.com/wfraser/dc4): Rust rewrite of dc(1)
- [eva](https://github.com/nerdypepper/eva): calculater REPL similar to bc(1)
- [cmp-rpncalc](https://github.com/PhilRunninger/cmp-rpncalc): RPN calculator as nvim-cmp source.
