# perm

A coreutils-esque tool that generates all permutations of a set of inputs.

## Usage

Elements to permutate are read from stdin.
Permutations are delivered over stdout.  

> [!CAUTION]  
> The amount of permutations grows by $$ \frac{n!}{(n-k)!} $$
> Avoid generating permutations of large sets.  
> Avoid using this in a loop.

### Arguments

| Argument         | Shortform | Explanation                                       | Default       |
|------------------|-----------|---------------------------------------------------|---------------|
| --k              | -k        | Amount of elements in one permutation             | Element count |
| --delim          | -d        | Delimiter between input elements, read from stdin | "\n"          |
| --elem-separator | -e        | Separator between elements in a permutation       | ""            |
| --separator      | -s        | Separator between permutations                    | "\n"          |

### Examples
```bash
$ echo -e "a\nb\nc" | perm
abc
acb
bac
bca
cab
cba
```

```bash
$ echo "abc" | perm -d ""
abc
acb
bac
bca
cab
cba
```

```bash
$ echo "me myself I" | perm -d " " -e " "
me myself I
me I myself
myself me I
myself I me
I me myself
I myself me
```

## Installation

```bash
$ cargo install --git https://github.com/Sumandora/perm
```