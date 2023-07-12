# There's hashmaps and bitwise stuff you can do too...

def part_2(inp: str) -> int:
    def is_dupl(sec: str) -> bool:
        for i in range(len(sec)):
            for j in range(len(sec)):
                if i == j: continue
                if sec[i] == sec[j]: return True
        return False

    AMT = 14
    for i in range(AMT, len(inp)):
        if not is_dupl(inp[i - AMT : i]):
            return i
    return -1

if __name__ == "__main__":
    with open("../inputs/day06.txt") as f:
        print(part_2(f.read()))
