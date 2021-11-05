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

Simple JIT-compiled functional programming language.

## Example

Example implementation of Sieve of Eratosthenes written in Rock

```c
memset = (vec: vec, val: number, n: number): vec => {
	i = 0
	while i < n {
		vec = vecset(vec, i, val)
		i = i + 1
	}
	vec
}

sieve = (n: number): void => {
	v = vecnew()
	prime = memset(v, 1, n + 1)

	p = 2

	while p * p <= n {
		if vecget(prime, p) == 1 {
			i = p * p
			while i <= n {
				prime = vecset(prime, i, 0)
				i = i + p
			}
		}

		p = p + 1
	}

	p = 2

	while p <= n {
		if vecget(prime, p) == 1 {
			print(string(p))
			print("\n")
		}

		p = p + 1
	}
}

sieve(100)
```

## License

This project is licensed under the terms of the MIT license.
