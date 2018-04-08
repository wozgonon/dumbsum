# dumbsum

A program that does nothing more than add up a stream of numbers.
The program works with 64 bit foating point numbers) and the program is intended for use in shell scripting.

To add a stream of numbers is such a simple operation and something I have wanted to do in various bash scripts recently
but there does not seem to be a simple straight for [unix filter](https://en.wikipedia.org/wiki/Filter_(software)) for this purpose.
So I have tried using various techniques, including those suggested [here](https://stackoverflow.com/questions/450799/shell-command-to-sum-integers-one-per-line).

Written in the [Rust programming language](https://www.rust-lang.org) this program ought to be quick and
since it uses the [Kahan summation algorithm](https://en.wikipedia.org/wiki/Kahan_summation_algorithm) it also ought to
significantly reduce the numerical error compared with a niave implementation.

## To build

Install the latest version of [Rust](https://www.rust-lang.org) and build using 'cargo':

$ cargo install

## Examples using [bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell)):

Add all the numbers from 1 to 10:

```bash
$ seq 1 10 | dumbsum
55
```
Add up a list of numbers held in a file called abc.txt:

```bash
$ cat abc.txt | dumbsum
```

Add a list of numbers entered as a [Here document](https://en.wikipedia.org/wiki/Here_document):

```bash
$ dumbsum <<EOF
> 11
> 22
> 33
> 44
> 55
> EOF
165
```

Using a [Here string](https://en.wikipedia.org/wiki/Here_document):

```bash
$ dumbsum <<< '1
> 2
> 3
> 4
> 5'
15
```

Add numbers separated by newlines:

```bash
$ printf "1\n2" | dumbsum
3
```

```bash
$ printf "101\n202" | dumbsum
303
```

Add a sequence of space separated numbers:

```bash
$ echo 15 24 33 42 51 | sed 's/ /\n/g' | dumbsum
165
```

Add up all the numbers returned by the disk usage (du) command:

```bash
$ du  | cut -f 1 | dumbsum
103022
```
