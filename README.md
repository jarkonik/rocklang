# Rock

[![CI](https://github.com/jarkonik/rocklang/actions/workflows/main.yml/badge.svg)](https://github.com/jarkonik/rocklang/actions/workflows/main.yml)

<img src="./rock.svg" width="100" height="100">

Simple JIT-compiled functional programming language.

## Example

Example implementation of Sieve of Eratosthenes written in Rock

```
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
