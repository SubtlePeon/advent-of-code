from typing import List

# In hindsight, I should have transposed the lines

def part_1(inp: str) -> str:
    inp = inp.split("\n")
    # ignore starting blank lines
    while inp[0] == "":
        inp.pop(0)
    # parse crate positions
    stacks = []
    for _ in range(9):
        stacks.append([])

    for _ in range(8):
        line = inp.pop(0)
        for i in range(9):
            if line[ 4 * i : 4 * i + 3 ] != "   ":
                stacks[i].insert(0, line[ 4 * i + 1 ])

    # ignore 2 lines
    for _ in range(2):
        inp.pop(0)

    # interpret instructions
    for line in inp:
        if line == "": continue
        _, count, _, fro, _ , to = line.split(" ")
        count, fro, to = int(count), int(fro), int(to)
        for _ in range(count):
            stacks[to - 1].append(stacks[fro - 1].pop())

    # get answer
    ans = ""
    for e in stacks:
        ans += e[-1]
    return ans

def part_2(inp: str) -> str:
    inp = inp.split("\n")
    # ignore starting blank lines
    while inp[0] == "":
        inp.pop(0)
    # parse crate positions
    stacks = []
    for _ in range(9):
        stacks.append([])

    # while not inp[0].beginswith(" 1")
    for _ in range(8):
        line = inp.pop(0)
        for i in range(9):
            if line[ 4 * i : 4 * i + 3 ] != "   ":
                stacks[i].insert(0, line[ 4 * i + 1 ])

    # ignore 2 lines
    for _ in range(2):
        inp.pop(0)

    # interpret instructions
    for line in inp:
        if line == "": continue
        _, count, _, fro, _ , to = line.split(" ")
        count, fro, to = int(count), int(fro), int(to)
        tmp = []
        for _ in range(count):
            tmp.append(stacks[fro - 1].pop())
        for item in reversed(tmp):
            stacks[to - 1].append(item)

    # get answer
    ans = ""
    for e in stacks:
        ans += e[-1]
    return ans

if __name__ == "__main__":
    with open("../inputs/day05.txt") as f:
        inp = f.read()
    print(part_2(inp))
