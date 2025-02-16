# arceos-apps

[![CI](https://github.com/arceos-org/arceos-apps/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/arceos-org/arceos-apps/actions/workflows/build.yml)
[![CI](https://github.com/arceos-org/arceos-apps/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/arceos-org/arceos-apps/actions/workflows/test.yml)

Example apps for [ArceOS](https://github.com/arceos-org/arceos).

## Quick Start

### 1. Install Build Dependencies

Install [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) to use `rust-objcopy` and `rust-objdump` tools:

```bash
cargo install cargo-binutils
```

Download ArceOS source code:

```bash
./scripts/get_deps.sh
```

The ArceOS repository will be cloned into `.arceos`.
You can also skip this step by specifying the `AX_ROOT` parameter when running the `make` command.

#### Dependencies for C apps

Install `libclang-dev`:

```bash
sudo apt install libclang-dev
```

Download & install [musl](https://musl.cc) toolchains:

```bash
# download
wget https://musl.cc/aarch64-linux-musl-cross.tgz
wget https://musl.cc/riscv64-linux-musl-cross.tgz
wget https://musl.cc/x86_64-linux-musl-cross.tgz
wget https://github.com/LoongsonLab/oscomp-toolchains-for-oskernel/releases/download/gcc-13.2.0-loongarch64/gcc-13.2.0-loongarch64-linux-gnu.tgz
wget https://github.com/LoongsonLab/oscomp-toolchains-for-oskernel/raw/refs/heads/main/musl-loongarch64-1.2.2.tgz
# install
tar zxf aarch64-linux-musl-cross.tgz
tar zxf riscv64-linux-musl-cross.tgz
tar zxf x86_64-linux-musl-cross.tgz
# exec below command in bash OR add below info in ~/.bashrc
export PATH=`pwd`/x86_64-linux-musl-cross/bin:`pwd`/aarch64-linux-musl-cross/bin:`pwd`/riscv64-linux-musl-cross/bin:`pwd`/gcc-13.2.0-loongarch64-linux-gnu/bin:`pwd`/musl-loongarch64-1.2.2/bin:$PATH
```

### 2. Build & Run

```bash
make A=path/to/app ARCH=<arch> LOG=<log>
```

Where `path/to/app` is the relative path to the application.

`<arch>` should be one of `riscv64`, `aarch64`, `x86_64`, `loongarch64`.

`<log>` should be one of `off`, `error`, `warn`, `info`, `debug`, `trace`.

Other arguments are the same as ArceOS's [Makefile](https://github.com/arceos-org/arceos/blob/main/Makefile).

For example, to run the [httpserver](rust/net/httpserver/) on `qemu-system-aarch64` with 4 cores and log level `info`:

```bash
make A=rust/net/httpserver ARCH=aarch64 LOG=info SMP=4 run NET=y
```

Note that the `NET=y` argument is required to enable the network device in QEMU. These arguments (`BLK`, `GRAPHIC`, etc.) only take effect at runtime not build time.

## List of Rust Apps

| App | `axstd` features | Extra modules | Description |
|-|-|-|-|
| [helloworld](rust/helloworld/) | | | A minimal app that just prints a string |
| [exception](rust/exception/) | | | Exception handling test |
| [memtest](rust/memtest/) | alloc | axalloc | Dynamic memory allocation test |
| [display](rust/display/) | display | axdriver, axdisplay | Graphic/GUI test |
| [yield](rust/task/yield/) | multitask | axalloc, axtask | Multi-threaded yielding test |
| [sleep](rust/task/sleep/) | multitask, irq | axalloc, axtask | Thread sleeping test |
| [parallel](rust/task/parallel/) | alloc, multitask | axalloc, axtask | Parallel computing test (to test synchronization & mutex) |
| [priority](rust/task/priority/) | alloc, multitask | axalloc, axtask | Task priority test |
| [tls](rust/task/tls/) | alloc, multitask, tls | axalloc, axtask | Thread local storage test |
| [shell](rust/fs/shell/) | alloc, fs | axalloc, axdriver, axfs | A simple shell that responds to filesystem operations |
| [httpclient](rust/net/httpclient/) | net | axalloc, axdriver, axnet | A simple client that sends an HTTP request and then prints the response |
| [udpserver](rust/net/udpserver/) | net | axalloc, axdriver, axnet | A single-threaded echo server using UDP protocol |
| [echoserver](rust/net/echoserver/) | alloc, multitask, net | axalloc, axdriver, axnet, axtask | A multi-threaded TCP server that reverses messages sent by the client |
| [httpserver](rust/net/httpserver/) | alloc, multitask, net | axalloc, axdriver, axnet, axtask | A multi-threaded HTTP server that serves a static web page |
| [bwbench](rust/net/bwbench/) | net | axalloc, axdriver, axnet | Network bandwidth benchmark |

## List of C Apps

| App | `axlibc` features | Extra modules | Description |
|-|-|-|-|
| [helloworld](c/helloworld/) | | | A minimal C app that just prints a string |
| [memtest](c/memtest/) | alloc | axalloc | Dynamic memory allocation test in C |
| [pthread_basic](c/pthread/basic/) | alloc, multitask | axalloc, axtask | Basic pthread test (create, join, exit, and mutex) |
| [pthread_parallel](c/pthread/parallel/) | alloc, multitask | axalloc, axtask | Parallel computing test in C |
| [pthread_sleep](c/pthread/sleep/) | alloc, multitask, irq | axalloc, axtask | Thread sleeping test in C |
| [pthread_pipe](c/pthread/pipe/) | alloc, multitask, pipe | axalloc, axtask | Multi-thread communication using pipe |
| [httpclient](c/httpclient/) | alloc, net | axalloc, axdriver, axnet | A simple client that sends an HTTP request and then prints the response |
| [udpserver](c/udpserver/) | alloc, net | axalloc, axdriver, axnet | A single-threaded echo server using UDP protocol |
| [httpserver](c/httpserver/) | alloc, net | axalloc, axdriver, axnet | A single-threaded HTTP server that serves a static web page |
| [sqlite3](c/sqlite3/) | fp_simd, alloc, fs | axalloc, axdriver, axfs | Porting of [SQLite3](https://sqlite.org/index.html) |
| [iperf](c/iperf/) | fp_simd, alloc, fs, net, select | axalloc, axdriver, axfs, axnet | Porting of [iPerf3](https://iperf.fr/) |
| [redis](c/redis/) | fp_simd, alloc, irq, multitask, fs, net, pipe, epoll | axalloc, axdriver, axtask, axfs, axnet | Porting of [Redis](https://redis.io/) |
