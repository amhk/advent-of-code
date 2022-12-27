#!/usr/bin/env python
from collections import defaultdict
import argparse
import re


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=argparse.FileType("r"))
    args = parser.parse_args()

    colors = defaultdict(lambda: "#fff0b2")
    colors["root"] = "#ef6c00"
    colors["humn"] = "#558b2f"

    print("digraph G {")
    print('    node [shape="rect" style="filled" ordering="out"];')
    for line in args.input.readlines():
        m = re.match(r"^(....): (\d+)", line)
        if m:
            id = m.group(1)
            value = m.group(2)
            print(f'    {id} [label="{id}\\n{value}" fillcolor="{colors[id]}"];')
            continue

        m = re.match(r"^(....): (....) (.) (....)", line)
        if m:
            id = m.group(1)
            lhs = m.group(2)
            op = m.group(3)
            rhs = m.group(4)
            print(f'    {id} [label="{id}\\n{op}" fillcolor="{colors[id]}"];')
            print(f"    {id} -> {lhs};")
            print(f"    {id} -> {rhs};")
            continue

        raise Exception(f"bad input: {line}")
    print("}")


if __name__ == "__main__":
    main()
