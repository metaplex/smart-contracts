# Metaplex-tests crate

To run tests just do `cargo test-bpf`

Also you can see that we have here *.so file of programs which we are depends from.

We move it here because cargo for some reason doesn't create *.so files of dependent programs when we run `cargo build-bpf`.

There is [possibility](https://docs.rs/solana-program-test/1.8.2/solana_program_test/struct.ProgramTest.html#method.add_program) to add programs to the test environment by setting `process_instruction` methods but it didn't work out