"""
        The sum-of-subsets problem states that a set of positive integers, and a
        value M, determine all possible subsets of the given set whose summation sum
        equal to given M.

        Summation of the chosen numbers must be equal to given number M and one number
        can be used only once.
"""

def generate_subsets(nums, value):
    subsets = []
    path = []
    beg = 0
    backtrack_all_path(subsets, path, nums, beg, value)
    return subsets

def backtrack_all_path(paths, path, nums, beg, value):
    if sum(path) == value:
        paths.append(path.copy())
        return
    if sum(path) > value or sum(path) + sum(nums[beg:]) < value:
        return
    for i in range(beg, len(nums)):
        path.append(nums[i])
        backtrack_all_path(paths, path, nums, i+1, value)
        path.pop()

nums = [9, 3, 34, 4, 12, 5, 2]
max_sum = 9
result = generate_subsets(nums, max_sum)
print(*result)