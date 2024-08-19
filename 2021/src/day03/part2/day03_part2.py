from collections import defaultdict
from copy import deepcopy
from typing import List
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    def most_common(iterable, cmp_key):
        elem_occurences = defaultdict(lambda: 0)
        for elem in iterable:
            elem_occurences[elem] += 1

        (most_common_elem, _occurences) = max(
            elem_occurences.items(), key=cmp_key
        )
        return most_common_elem

    def bits2decimal(bits) -> int:
        return int(bits, base=2)

    bits_matrix = puzzle_input.splitlines()

    row_count = len(bits_matrix)

    def rate(cmp_key):
        kept_row_indices = list(range(0, row_count))

        for col in zip(*bits_matrix):
            keep_bit = most_common(
                map(lambda row_idx: col[row_idx], kept_row_indices),
                cmp_key,
            )
            # TODO: improve performance: unnecessary collecting into new list every iteration.
            kept_row_indices = list(
                filter(
                    lambda row_idx: col[row_idx] == keep_bit,
                    kept_row_indices,
                )
            )

            match kept_row_indices:
                case [row_idx]:
                    return bits_matrix[row_idx]

        raise Exception(
            "unreachable: expected there to be one leftover number eventually"
        )

    def by_occurences_then_bit_increasing(item):
        (bit, occurences) = item
        return (occurences, int(bit))

    def by_occurences_then_bit_decreasing(item):
        (bit, occurences) = item
        return (-occurences, -int(bit))

    oxygen_generator_rating_bits = rate(by_occurences_then_bit_increasing)
    co2_scrubber_rating_bits = rate(by_occurences_then_bit_decreasing)

    oxygen_generator_rating = bits2decimal(oxygen_generator_rating_bits)
    co2_scrubber_rating = bits2decimal(co2_scrubber_rating_bits)

    life_support_rating = oxygen_generator_rating * co2_scrubber_rating

    return str(life_support_rating)


if __name__ == "__main__":
    run(solve)
