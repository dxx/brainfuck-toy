# brainfuck-toy

This project is a toy that interpret and compile brainfuck code.

## Run

### Interpreter

```shell
❯ cargo run --release --bin interpreter ./bf/hello_world.bf
Hello World!
```

### Optimized Interpreter

```shell
❯ cargo run --release --bin interpreter_it ./bf/hello_world.bf
Hello World!
```

### JIT

Support:

* aarch64
* x64 (Linux only)

```shell
❯ cargo run --release --bin jit ./bf/hello_world.bf
Hello World!
```
