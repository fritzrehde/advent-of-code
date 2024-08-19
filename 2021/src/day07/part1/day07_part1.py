from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    crab_horizontal_positions = list(
        map(int, puzzle_input.rstrip("\n").split(","))
    )

    min_pos = min(crab_horizontal_positions)
    max_pos = max(crab_horizontal_positions)

    def required_fuel(pos: int) -> int:
        """
        The amount of fuel required for all crabes to move to the given
        position.
        """
        return sum(
            abs(crab_pos - pos) for crab_pos in crab_horizontal_positions
        )

    min_fuel = min(
        required_fuel(possible_pos)
        for possible_pos in range(min_pos, max_pos + 1)
    )

    return str(min_fuel)


if __name__ == "__main__":
    run(solve)
