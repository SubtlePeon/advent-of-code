def part_1(inp: str) -> int:
    points = 0
    for line in inp.split("\n"):
        if line == "": continue
        [a, b] = line.split(" ")
        opp_ind = ["A", "B", "C"].index(a)
        you_ind = ["X", "Y", "Z"].index(b)
        points += you_ind + 1
        # determine win/loss/draw
        if opp_ind == you_ind:
            # draw
            points += 3
        elif opp_ind == (you_ind - 1) % 3:
            # win
            points += 6
    return points

def part_2(inp: str) -> int:
    points = 0
    for line in inp.split("\n"):
        if line == "": continue
        [a, b] = line.split(" ")
        opp_ind = ["A", "B", "C"].index(a)
        game_res = ["X", "Y", "Z"].index(b)

        points += game_res * 3
        points += (opp_ind + game_res - 1) % 3 + 1
    return points

if __name__ == "__main__":
    with open("../inputs/day02.txt") as f:
        inp = f.read()
    res = part_2(inp)
    print(res)
