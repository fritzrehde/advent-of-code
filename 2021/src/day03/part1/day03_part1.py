from collections import defaultdict
from typing import List
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    def most_and_least_common(iterable):
        elem_occurences = defaultdict(lambda: 0)
        for elem in iterable:
            elem_occurences[elem] += 1

        def by_occurences(item):
            (_elem, occurences) = item
            return occurences

        # TODO: improve performance: find min and max in single pass.
        (most_common_elem, _occurences) = max(
            elem_occurences.items(), key=by_occurences
        )
        (least_common_elem, _occurences) = min(
            elem_occurences.items(), key=by_occurences
        )
        return (most_common_elem, least_common_elem)

    def bits2decimal(bits) -> int:
        return int("".join(bits), base=2)

    bits_matrix = puzzle_input.splitlines()

    (gamma_rate_bits, epsilon_rate_bits) = zip(
        *(most_and_least_common(col) for col in zip(*bits_matrix))
    )

    gamma_rate = bits2decimal(gamma_rate_bits)
    epsilon_rate = bits2decimal(epsilon_rate_bits)

    power_consumption = gamma_rate * epsilon_rate

    return str(power_consumption)


if __name__ == "__main__":
    run(solve)
