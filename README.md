# r0-puzzles

This repo contains a collection of small puzzles involving the [RISC Zero VM](https://dev.risczero.com/api/next/zkvm/). These puzzles involve digging into and understanding [RISC-V ELF binaries](https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-elf.adoc). Each puzzle utilizes different APIs of the RISC Zero VM, and roughly increases in difficulty each puzzle.

These puzzles follow a similar format to [weikengchen's CTF challenge](https://github.com/weikengchen/zkctf-r0-season1), so if you are strugging to analyze the binaries, [their article](https://l2ivresearch.substack.com/i/138905838/look-at-the-risc-v) with a section on decompiling ELF binaries might be a helpful entry. There is no requirement to use any specific software to solve these puzzles, however.

All puzzles are setup as tests, so to run all:

```console
cargo test
```

Or you can run specific puzzles at a time with:

```console
cargo test puzzle1
```

and replace the number with the one you want to run specifically.

All code written for the puzzle should be written in [./host/src/lib.rs](./host/src/lib.rs), and there will be instructions in that file of what to modify! There is some template code to hint towards how to setup the `ExecutorEnv`, but feel free to change anything above the line indicated not to go below.
