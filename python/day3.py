import numpy as np
import re
import os

pat = re.compile(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')


def part1(pth):
    arr = np.zeros((1000, 1000))
    with open(pth) as f:
        for line in f:
            x, y, w, h = map(int, pat.match(line).group(2, 3, 4, 5))
            arr[y:y+h, x:x+w] += 1

    return np.count_nonzero(arr > 1)


def part2(pth):
    arr = np.zeros((1000, 1000))
    with open(path) as f:
        for line in f:
            claim_id, x, y, w, h = map(int, pat.match(line).group(1, 2, 3, 4, 5))
            slice = arr[y:y+h, x:x+w]
            if np.any(slice != 0):
                for n in np.unique(slice):
                    if n not in (0, -1):
                        arr[arr == n] = -1
                slice[:] = -1
            else:
                slice[:] = claim_id

    return np.unique(arr)


if __name__ == '__main__':
    path = os.path.join(os.getcwd(), "input/2018/day3.txt")
    print(part1(path))
    print(part2(path))
