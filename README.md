# Rock

⚠️The project is at a very early stage, lots of things will not work.

[![Github all releases](https://img.shields.io/github/downloads/jarkonik/rocklang/total.svg)](https://GitHub.com/jarkonik/rocklang/releases/)
[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/donate?hosted_button_id=Y25KDXY4LJYQ2)
[![Continuous integration](https://github.com/jarkonik/rocklang/actions/workflows/main.yml/badge.svg)](https://github.com/jarkonik/rocklang/actions/workflows/main.yml)
[![codecov](https://codecov.io/gh/jarkonik/rocklang/branch/main/graph/badge.svg?token=DW07IRWGG0)](https://codecov.io/gh/jarkonik/rocklang)
[![Rocklang
Discord](https://badgen.net/discord/members/NK3baHRTve)](https://discord.gg/NK3baHRTve)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<img src="./rock.svg" width="100" height="100">

JIT-compiled programming language.

## Getting started

### Linux

1. Copy and paste the following command in a shell:
```
sudo sh -c 'curl -L https://github.com/jarkonik/rocklang/releases/latest/download/rocklang-ubuntu-latest -o /usr/local/bin/rocklang && chmod +x /usr/local/bin/rocklang'
```
2. Create a file named `main.rck` in a directory of your choice, with following
   content:
```
print("Hello from rocklang")
```
3. While being in the same directory, that you've created the source file in, run `rocklang main.rck` from a shell.
4. You should see text `Hello from rocklang` printed in your terminal.

### Windows

1. [Download and run the installer.](https://github.com/jarkonik/rocklang/releases/latest/download/rocklang-windows-latest.msi)
2. Create a file named `main.rck` in a directory of your choice, with the following
   content:
```
print("Hello from rocklang")
```
3. While being in the same directory, that you've created the source file in, run `rocklang main.rck` from PowerShell or
   Command Prompt.
4. You should see text `Hello from rocklang` printed in your terminal.

### MacOS

1. Copy and paste the following command in a shell:
```
sudo sh -c 'curl -L https://github.com/jarkonik/rocklang/releases/latest/download/rocklang-macos-latest -o /usr/local/bin/rocklang && chmod +x /usr/local/bin/rocklang'
```
2. Create a file named `main.rck` in a directory of your choice, with the following
   content:
```
print("Hello from rocklang")
```
3. While being in the same directory, that you've created the source file in, run `rocklang main.rck` from a shell.
4. You should see text `Hello from rocklang` printed in your terminal.

## Example

Sample implementation of Sieve of Eratosthenes written in Rock

```c
mem_set = (vec: vec, val: number, n: number): vec => {
	i = 0
	while i < n {
		vec_set(vec, i, val)
		i = i + 1
	}
	vec
}

sieve = (n: number): void => {
	v = vec_new()
	prime = mem_set(v, 1, n + 1)

	p = 2

	while p * p <= n {
		if vec_get(prime, p) == 1 {
			i = p * p
			while i <= n {
				vec_set(prime, i, 0)
				i = i + p
			}
		}

		p = p + 1
	}

	p = 2

	while p <= n {
		if vec_get(prime, p) == 1 {
			print(string(p))
			print("\n")
		}

		p = p + 1
	}
}

sieve(10)
```

## Building from source
1. Install Rust compiler that supports Rust Edition 2021, along with `cargo` tool, in your favorite fashion.
2. Install llvm 13
3. Run `cargo build` to build binaries or `cargo run examples/sieve.rck` to run a sample program.

## License

This project is licensed under the terms of the MIT license.
