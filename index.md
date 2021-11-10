## Examples

### Sieve of Eratosthenes
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

sieve(10)
```
