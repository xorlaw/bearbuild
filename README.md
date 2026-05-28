# bearbuild

A less-sucky, modern and quick Ninja compatible build system in Rust. It is designed to be a smaller, modern, and memory safe
replacement for the Meson build system.


## features

- Fully memory safe
- Actually human readable error messages
- Simple glob based source discovery
- Smaller codebase than alternatives
- Single configuration file

## installing

### from source

You will need Rust and Cargo installed to compile Bearbuild from source. 

```bash
git clone https://github.com/xorlaw/bearbuild
cd bearbuild
cargo build --release
```
The release build profile will create a leaner and more optimized binary. However, the release binary takes noticeably longer to
compile than some other options. If you are on a lower end machine, you can compile using the debug profile which is slower but easier
to compile. You can compile with the debug profile by simply adding the `debug` flag when building with Cargo.

The binary will be at `target/release/bearbuild`. Copy it somewhere on your PATH:
 
```bash
cp target/release/bearbuild ~/.local/bin/
```

### usage

#### 1. create your project layout

```
project/
  bear.toml
  src/
    main.c
    util.c
  include/
    util.h
```

#### 2. write a basic `bear.toml` file

```toml
[project]
name    = "myproject"
version = "0.1.0"
 
[build]
compiler = "gcc"
std      = "c11"
sources  = ["src/*.c"]
includes = ["include"]
flags    = ["-Wall", "-Wextra"]
 
[output]
binary = "myproject"
```

#### 3. build

```bash
bearbuild       # generates build.ninja
ninja           # compiles your project
./project     # run it
```
### bear.toml reference
 
#### `[project]`
 
| Key | Type | Required | Description |
|---|---|---|---|
| `name` | string | yes | Name of the project |
| `version` | string | yes | Version string (e.g. `"1.0.0"`) |
 
#### `[build]`
 
| Key | Type | Required | Description |
|---|---|---|---|
| `compiler` | string | yes | Compiler binary to use (e.g. `"gcc"`, `"clang"`) |
| `std` | string | yes | C standard to pass to `-std=` (e.g. `"c11"`, `"c99"`) |
| `sources` | list | yes | Source file globs or paths (e.g. `["src/*.c"]`) |
| `includes` | list | no | Include directories, passed as `-I` flags |
| `flags` | list | no | Extra compiler flags (e.g. `["-Wall", "-O2"]`) |
 
##### `[deps]` (optional)
 
 
| Key | Type | Description |
|---|---|---|
| `pkgs` | list | Package names as passed to `pkg-config` |
 
Example:
 
```toml
[deps]
pkgs = ["libpng", "zlib"]
```
 
 
#### `[output]`
 
| Key | Type | Required | Description |
|---|---|---|---|
| `binary` | string | yes | Name of the output binary |
 

## licensing

Licensed under The Unlicense.
