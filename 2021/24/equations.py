#!/usr/bin/env python3

stack = []
for (i, filename) in enumerate(map(lambda x: f"input-{x:02d}.txt", range(0, 14))):
    with open(filename) as f:
        # index 4: div z <int>
        # index 5: add x <int>
        # index 15: add y <int>
        lines = f.readlines()
        a = int(lines[4].split()[2])
        b = int(lines[5].split()[2])
        c = int(lines[15].split()[2])

        if a == 1:  # push operation
            stack.append((i, a, b, c))
        elif a == 26:  # pop operation
            # (Digit Number Of Push) + (C of Push) + (B of Pop) = (Digit Number of Pop)
            (push_i, push_a, push_b, push_c) = stack.pop()
            constant = push_c + b
            print(f"input[{push_i}] + {constant} = input[{i}]")
        else:
            raise Exception("invalid state")
assert len(stack) == 0
