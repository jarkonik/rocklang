# Rock

<img src="./rock.svg" width="100" height="100">

Simple JIT-compiled functional programming language.

## Example

Example implementation of Sieve of Eratosthenes written in Rock

```
memset = (arr, val, n) => {
	i = 0
	while i < n {
		arr = arrset(arr, i, val)
		i = i + 1
	}
	arr
}

sieve = (n) => {
	prime = memset(arrnew(), true, n + 1)

	p = 2

	while p * p <= n {
		if arrget(prime, p) {
			i = p * p
			while i <= n {
				prime = arrset(prime, i, false)
				i = i + p
			}
		}

		p = p + 1
	}

	p = 2

	while p <= n {
		if arrget(prime, p) {
			print(p)
			print("\n")
		}

		p = p + 1
	}
}

sieve(7)
```

## License

This project is licensed under the terms of the MIT license.
