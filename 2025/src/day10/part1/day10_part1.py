from typing import Iterable
from typing import TypeVar
from typing import Tuple
from typing import Deque
from typing import Dict
from collections import deque
from typing import Set
from typing import DefaultDict
from typing import List
from collections import defaultdict
from puzzle_utils import puzzle, run

T = TypeVar("T")


@puzzle
def solve(puzzle_input: str) -> str:
    def parse_line(line: str):
        indicator_lights_end_idx = line.find("]")
        joltages_start_idx = line.find("{")

        indicator_lights = tuple(
            light == "#" for light in line[1:indicator_lights_end_idx]
        )
        # each button [a, b, c] toggles indicators a, b, c
        buttons = [
            list(
                map(
                    int,
                    # remove leading "(" and trailing ")"
                    (button[1:-1]).split(","),
                )
            )
            for button in line[
                indicator_lights_end_idx + 1 : joltages_start_idx
            ]
            .strip()
            .split()
        ]
        joltages = list(map(int, line[joltages_start_idx + 1 : -1].split(",")))

        return indicator_lights, buttons, joltages

    def all_indicator_lights_states(num_lights: int):

        def backtrack(acc: List[bool], i: int):
            if i == num_lights:
                yield tuple(acc)
                return

            for append in (True, False):
                acc.append(append)
                yield from backtrack(acc, i + 1)
                acc.pop()

        yield from backtrack(acc=[], i=0)

    def shortest_paths(
        neighbours: DefaultDict[T, Set[T]], nodes: Iterable[T], start_node: T
    ) -> Dict[T, int | None]:
        # invariant: only gets set once on first visit, that will be SP to it
        node_to_dist: Dict[T, int | None] = {node: None for node in nodes}

        q: Deque[Tuple[T, int]] = deque()
        visited = set()

        q.append((start_node, 0))

        while len(q) > 0:
            node, dist = q.popleft()

            if node in visited:
                continue
            visited.add(node)
            node_to_dist[node] = dist

            for neighbour in neighbours[node]:
                q.append((neighbour, dist + 1))

        return node_to_dist

    def press_button(
        light_state: Tuple[bool, ...], button: List[int]
    ) -> Tuple[bool, ...]:
        # returns end state
        light_state_mut = list(light_state)
        for light_idx in button:
            light_state_mut[light_idx] = not light_state_mut[light_idx]
        return tuple(light_state_mut)

    def min_presses_to_reach_light_state(
        light_end_state: Tuple[bool, ...],
        buttons: List[List[int]],
        joltages: List[int],
    ) -> int:
        # strategy:
        # generate a directed unweighted graph where each possible indicator
        # light diagram represents a node and button presses represent
        # edges, and find shortest path with bfs

        num_lights = len(light_end_state)
        start_node = tuple(False for _ in range(num_lights))
        end_node = light_end_state

        neighbours: DefaultDict[Tuple[bool, ...], Set[Tuple[bool, ...]]] = (
            defaultdict(lambda: set())
        )

        nodes = list(all_indicator_lights_states(num_lights))

        for light_node in nodes:
            for button in buttons:
                neighbours[light_node].add(press_button(light_node, button))

        dist_from_start = shortest_paths(neighbours, nodes, start_node)

        end_dist = dist_from_start[end_node]
        assert end_dist is not None
        return end_dist

    res = sum(
        min_presses_to_reach_light_state(*parse_line(line))
        for line in puzzle_input.splitlines()
    )

    return str(res)


if __name__ == "__main__":
    run(solve)
