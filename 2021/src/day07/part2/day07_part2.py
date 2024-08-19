from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    crab_horizontal_positions = list(
        map(int, puzzle_input.rstrip("\n").split(","))
    )

    min_pos = min(crab_horizontal_positions)
    max_pos = max(crab_horizontal_positions)

    def sum_first_n(n: int) -> int:
        """
        Return the sum of the first 1 to n (both inclusive). Assume n is even.
        """
        # Arithmetic series:
        # sum(1 to n)
        # = 1 + 2 + ... + (n-1) + n
        # = (1 + n) + (2 + (n-1)) + ...
        # = (1 + n) + (1 + n) + ...
        # = (n/2)(1 + n)
        return (n // 2) * (1 + n)

    def distance_fuel_cost(distance: int) -> int:
        """
        The amount of fuel it costs to travel the given distance/number of
        steps.
        """
        # NOTE: Technically, the arithmetic series works for any n (even odd
        # numbers), but we want to avoid lossy float division.
        if distance % 2 == 0:
            return sum_first_n(distance)
        else:
            return sum_first_n(distance - 1) + distance

    def required_fuel(pos: int) -> int:
        """
        The amount of fuel required for all crabes to move to the given
        position.
        """
        return sum(
            distance_fuel_cost(abs(crab_pos - pos))
            for crab_pos in crab_horizontal_positions
        )

    min_fuel = min(
        required_fuel(possible_pos)
        for possible_pos in range(min_pos, max_pos + 1)
    )

    return str(min_fuel)


if __name__ == "__main__":
    run(solve)
