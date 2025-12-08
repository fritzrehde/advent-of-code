from functools import reduce
from typing import Iterable
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    rows = [list(row.split()) for row in puzzle_input.splitlines()]
    num_rows = len(rows)
    num_cols = len(rows[0])

    def apply_op(op: str, args: Iterable[int]) -> int:
        match op:
            case "+":
                op_fn = lambda x, y: x + y
            case "*":
                op_fn = lambda x, y: x * y
            case _:
                raise Exception("unreachable")

        return reduce(op_fn, args)

    def eval_col(col_idx: int) -> int:
        col = [row[col_idx] for row in rows]
        op = col[-1]
        args = map(int, col[:-1])
        return apply_op(op, args)

    res = sum(eval_col(col_idx) for col_idx in range(0, num_cols))

    return str(res)


if __name__ == "__main__":
    run(solve)
