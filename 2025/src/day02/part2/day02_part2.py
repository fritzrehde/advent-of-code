from puzzle_utils import puzzle, run, dbg


@puzzle
def solve(puzzle_input: str) -> str:

    def is_repeated_pattern(num: str) -> bool:
        # try all repeating block sizes
        for block_len in range(1, len(num) // 2 + 1):
            (num_blocks, remainder) = divmod(len(num), block_len)
            if remainder != 0:
                continue

            block = num[:block_len]
            if block * num_blocks == num:
                return True

        return False

    def invalid_nums():
        for r in puzzle_input.split(","):
            # auto trims leading zeros
            a, b = map(int, r.split("-"))

            a_val, b_val = int(a), int(b)
            assert a_val <= b_val

            def all_invalid_in_range(a: int, b: int):
                for x in range(a, b + 1):
                    x_str = str(x)

                    if is_repeated_pattern(x_str):
                        yield x

            yield from all_invalid_in_range(a, b)

    return str(sum(invalid_nums(), start=0))


if __name__ == "__main__":
    run(solve)
