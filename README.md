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
