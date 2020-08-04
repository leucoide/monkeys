# Monkeys :see_no_evil: :hear_no_evil: :speak_no_evil:
---
Generate random templated data like:

```shell
monkeys '{n(1,100)} is a number between 1 and 100'
```

or generate a  random dataset:

```shell
echo "date,severity" > bugs.csv
monkeys '20{n(13,21)}-{n(1,13)}-{n(1,29)},{n(1,6)}' 200 >> bugs.csv
```

## Supported Expresssions

### Natural number {n}

Generates a random natural number. Can be called with or without
parameters [min, max). If called without parameters, `min` and
`max` values will be `0` and rust's `std::u32::MAX`.

Examples:

```shell
$ monkeys 'Hello {n}' 3

Hello 3701100621
Hello 132085837
Hello 3014903753

```

```shell
$ monkeys 'Hello {n(2,4)}' 3

Hello 2
Hello 3
Hello 3
```

### Floating point number {f}

Generates a random floating point number. Syntax is similar to `{n}`
with the addition of a third parameter, `round`, that limits the number of
decimals displayed and is currently limited to integer min and max values.
Default values are `0`, `std::f32::MAX as u32` and `2`.

Examples:

```shell
$ monkeys 'Hello {f}' 3

Hello 3208924672.00
Hello 1812630528.00
Hello 576724992.00

```

```shell
$ monkeys 'Hello {n(2,4,1)}' 3

Hello 2.0
Hello 3.4
Hello 3.8

```

### Sequential values {seq}

Generate a sequence of natural numbers. Parameters ar passed in
the form of a linear expression `$an [+ $b]` (`2n + 1` for example).
If no parameter is passed, the expression (`n + 1`) is considered.

Examples:

```shell
$ monkeys 'Hello {seq}' 3

Hello 1
Hello 2
Hello 3

```

```shell
$ monkeys 'Hello {seq(2n + 10)}' 3

Hello 10
Hello 12
Hello 14

```

