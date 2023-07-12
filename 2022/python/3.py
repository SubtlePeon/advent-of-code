def part_1(inp: str) -> int:
    every = []
    for line in inp.split("\n"):
        if line == "": continue
        l = len(line) // 2
        first = line[:l]
        second = line[l:]
        letter = {*first}.intersection({*second})
        every += [*letter]

    total = 0
    for e in every:
        if e.lower() == e:
            total += ord(e) - ord("a") + 1
        else:
            total += ord(e) - ord("A") + 27

    return total

def part_2(inp: str) -> int:
    every = []
    lines = [line for line in inp.split("\n") if line != ""]
    for i in range(0, len(lines), 3):
        letter = {*lines[i]}.intersection({*lines[i + 1]}).intersection({*lines[i + 2]})
        every += [*letter]

    total = 0
    for e in every:
        if e.lower() == e:
            total += ord(e) - ord("a") + 1
        else:
            total += ord(e) - ord("A") + 27

    return total

if __name__ == "__main__":
    with open("../inputs/day03.txt") as f:
        inp = f.read()
    print(part_1(inp))
