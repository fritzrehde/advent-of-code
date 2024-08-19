from puzzle_utils import puzzle, run


@puzzle
def solve(puzzle_input: str) -> str:
    timers = list(map(int, puzzle_input.rstrip("\n").split(",")))

    def tick(timer: int):
        if timer == 0:
            timer = 6
            child_timer = 8
            return [timer, child_timer]
        else:
            timer -= 1
            return [timer]

    DAYS = 80
    for _day in range(0, DAYS):
        new_timers = []
        for timer in timers:
            new_timers.extend(tick(timer))
        timers = new_timers

    total_fish = len(timers)

    return str(total_fish)


if __name__ == "__main__":
    run(solve)
