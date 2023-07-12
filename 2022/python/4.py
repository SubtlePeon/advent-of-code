from collections.abc import Iterable
from typing import List

def part_1(inp: str) -> int:
    def contains(range: List[int], e: List[int]) -> bool:
        [a, b] = range
        lo = min(a, b)
        hi = max(a, b)
        return e[0] >= lo and e[0] <= hi and e[1] >= lo and e[1] <= hi

    total = 0
    for line in inp.splitlines():
        if line == "": continue
        l = line.split(",")
        for i in range(len(l)):
            l[i] = [int(x) for x in l[i].split("-")];
        if contains(l[0], l[1]) or contains(l[1], l[0]):
            total += 1

    return total

def part_2(inp: str) -> int:
    def contains(range: List[int], e: List[int]) -> bool:
        [a, b] = range
        lo = min(a, b)
        hi = max(a, b)
        return e[0] >= lo and e[0] <= hi or e[1] >= lo and e[1] <= hi

    total = 0
    for line in inp.splitlines():
        if line == "": continue
        l = line.split(",")
        for i in range(len(l)):
            l[i] = [int(x) for x in l[i].split("-")];
        if contains(l[0], l[1]) or contains(l[1], l[0]):
            total += 1

    return total

if __name__ == "__main__":
    with open("../inputs/day04.txt") as f:
        inp = f.read()
    print(part_2(inp))
