<h1 align="center">Auditable MPC</h1>

This library provides the benchmarks for Auditable MPC. This uses marlin(https://github.com/scipr-lab/marlin) internally and builds on the codebase of marlin. 

## How to run?
```
docker build -t auditablempc .
```
This should run test cases for the servers.

Alternatively, you can use cargo/rustc to run the code.

```
cargo build
cargo test
```

The test automatically runs code for various grpahs that are shown in the paper. The enviroment variables, `NUM_SERVERS`, `NUM_CLIENTS`, `NUM_CONSTRAINTS` can be set to run the code for a specific instance. The circuits can be programmed in R1CS as specified in Marlin code. Refer to the code for an example. 
## License

This library is licensed under either of the following licenses, at your discretion.

 * [Apache License Version 2.0](LICENSE-APACHE)
 * [MIT License](LICENSE-MIT)

Unless you explicitly state otherwise, any contribution that you submit to this library shall be dual licensed as above (as defined in the Apache v2 License), without any additional terms or conditions.

[marlin]: https://ia.cr/2019/1047
