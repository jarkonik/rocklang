# Rock

Hobby programming language. Inspired by many languages that I have enjoyed programming in through my career. Dynamically typed and interpreted for now, but we'll see what future brings when it comes to that :)

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
