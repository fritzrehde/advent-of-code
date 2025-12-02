from puzzle_utils import puzzle, run, dbg


def digits(i: int) -> int:
    return len(str(i))


@puzzle
def solve(puzzle_input: str) -> str:
    def invalid_nums():
        for r in puzzle_input.split(","):
            # auto trims leading zeros
            a, b = map(int, r.split("-"))

            a_val, b_val = int(a), int(b)
            assert a_val <= b_val

            def all_invalid_in_range(a: int, b: int):
                for x in range(a, b + 1):
                    x_str = str(x)
                    if len(x_str) % 2 != 0:
                        # odd length is always valid
                        continue

                    middle = len(x_str) // 2
                    if x_str[:middle] == x_str[middle:]:
                        yield x

            yield from all_invalid_in_range(a, b)

    return str(sum(invalid_nums(), start=0))


if __name__ == "__main__":
    run(solve)
