from typing import Tuple, List
from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    points = [
        tuple(map(int, line.split(","))) for line in puzzle_input.splitlines()
    ]

    def rectangle_area(pt_a, pt_b) -> int:
        ax, ay = pt_a
        bx, by = pt_b

        dx = abs(ax - bx) + 1
        dy = abs(ay - by) + 1

        return dx * dy

    def all_rectangle_areas():
        for i, pt_a in enumerate(points):
            for pt_b in points[i + 1 :]:
                yield rectangle_area(pt_a, pt_b)

    max_rectangle_area = max(all_rectangle_areas())

    return str(max_rectangle_area)


if __name__ == "__main__":
    run(solve)
