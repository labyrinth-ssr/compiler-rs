docker pull maxxing/compiler-dev
docker run -it -w /root/compiler --rm -v /home/ssr/compiler/compiler:/root/compiler maxxing/compiler-dev bash
docker run -it -w /root/compiler --rm -v /home/ssr/compiler/compiler:/root/compiler com-exp bash
cargo run -- -koopa debug/hello.c -o hello.koopa
cargo run -- -koopa debug/unary.c -o unary.koopa
cargo run -- -riscv debug/unary.c -o unary.S
cargo run -- -riscv debug/hello.c -o hello.S