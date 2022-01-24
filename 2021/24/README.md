# Background

Let `input[i], 0 <= i < 14` be the input digits. The puzzle states that all
digits `x` must be `1 <= x <= 9`.

This
[Reddit thread](https://old.reddit.com/r/adventofcode/comments/rnejv5/2021_day_24_solutions/hps5hgw)
explains the ALU state machine and was used to implement `equations.py`.

The input (`input.txt`) has been split into its 14 parts
(`input-\d\d.txt`).`equations.py` gives the following equations:

```
input[2] + 2 = input[3]
input[6] + 5 = input[7]
input[5] - 5 = input[8]
input[9] + 1 = input[a]
input[4] + 6 = input[b]
input[1] + 7 = input[c]
input[0] - 4 = input[d]
```


# Part 1

Manually solve the equations for the maximum input value:

```
                    index: 0123456789abcd
input[2] + 2 = input[3] -> ..79..........
input[6] + 5 = input[7] -> ......49......
input[5] - 5 = input[8] -> .....9..4.....
input[9] + 1 = input[a] -> .........89...
input[4] + 6 = input[b] -> ....3......9..
input[1] + 7 = input[c] -> .2..........9.
input[0] - 4 = input[d] -> 9............5
                         = 92793949489995
```


# Part 2

Manually solve the equations for the minimum input value:

```
                    index: 0123456789abcd
input[2] + 2 = input[3] -> ..13..........
input[6] + 5 = input[7] -> ......16......
input[5] - 5 = input[8] -> .....6..1.....
input[9] + 1 = input[a] -> .........12...
input[4] + 6 = input[b] -> ....1......7..
input[1] + 7 = input[c] -> .1..........8.
input[0] - 4 = input[d] -> 5............1
                         = 51131616112781
```
