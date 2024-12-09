from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    puzzle_input = puzzle_input.strip()

    # "." corresponds to value -1
    empty = -1

    uncompressed = []
    id = 0
    for i, x in enumerate(map(int, puzzle_input)):
        if i % 2 == 0:
            # Files.
            uncompressed.extend([id] * x)
            id += 1
        else:
            # Free space.
            uncompressed.extend([empty] * x)

    n = len(uncompressed)

    l, r = 0, n - 1
    while l < r:
        # Go to next (rightwards) empty space '.'
        while l < r and uncompressed[l] != empty:
            l += 1

        # Go to next (leftwards) file space.
        while l < r and uncompressed[r] == empty:
            r -= 1

        # Swap.
        if l < r:
            uncompressed[l], uncompressed[r] = uncompressed[r], uncompressed[l]

    def non_empty(item):
        return item != empty

    res = sum(i * v for i, v in enumerate(filter(non_empty, uncompressed)))

    return str(res)


if __name__ == "__main__":
    run(solve)
