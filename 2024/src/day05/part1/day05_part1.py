from typing import List
from puzzle_utils import puzzle, run


def middle_val(l: List):
    assert len(l) % 2 == 1
    return l[len(l) // 2]


@puzzle
def solve(puzzle_input: str) -> str:
    lines = iter(puzzle_input.splitlines())

    # Parse rules.
    rules = []
    for line in lines:
        if line == "":
            break
        x, y = map(int, line.split("|"))
        rules.append((x, y))

    # Handle updates.
    sum = 0
    for line in lines:
        values = list(map(int, line.split(",")))
        indices = {v: i for (i, v) in enumerate(values)}
        correct_order = all(
            indices[x] < indices[y]
            for (x, y) in rules
            if x in values and y in values
        )
        if correct_order:
            sum += middle_val(values)

    return str(sum)


if __name__ == "__main__":
    run(solve)
