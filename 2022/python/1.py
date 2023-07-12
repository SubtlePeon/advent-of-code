def count_calories(puzzle: str) -> int:
    puzzle_list = puzzle.split("\n")
    
    max_so_far = [0] * 3  # How many we need
    so_far = 0
    for line in puzzle_list:
        if line == "":
            max_so_far = [*sorted(max_so_far + [so_far])][::-1][:len(max_so_far)]
            so_far = 0
        else:
            so_far += int(line)
    return sum(max_so_far)

if __name__ == "__main__":
    with open("../inputs/day01.txt") as f:
        print(count_calories(f.read()))
