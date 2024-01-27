import itertools
from typing import Sequence

def inverse_pairs(array: Sequence[int]) -> int:
    """Count the number of inverse pairs in an array.

    Args:
        array: A list of integers.

    Returns:
        The number of inverse pairs in the array.

    Raises:
        TypeError: If the array is not a list.
        ValueError: If the array is empty.
    """
    count = 0
    for i, j in itertools.combinations(range(len(array)), 2):
        if array[i] > array[j]:
            count += 1
    return count

input_arr = [1,2,3,4,5,6]

counts = []

for arr in itertools.permutations(input_arr):
    # for n in arr:
    #     print(f'{n},', end='')
    count = inverse_pairs(arr)
    # print('->', count)
    counts.append(count)

max_count = max(*counts)
prevalence = [0]*(max_count+1)
for count in counts:
    prevalence[count] += 1

print(prevalence)