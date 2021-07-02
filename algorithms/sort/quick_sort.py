'''
    quick sort
'''

def quick_sort(nums, beg, end):
    if beg+1 >= end:
        return
    mid = partition(nums, beg, end)
    quick_sort(nums, beg, mid)
    quick_sort(nums, mid+1, end)

def partition(nums, beg, end):
    left = []
    right = []
    n = nums[beg]
    for i in range(beg+1, end):
        if nums[i] < n:
            left.append(nums[i])
        else:
            right.append(nums[i])
    mid = beg+len(left)
    nums[beg:mid] = left.copy()
    nums[mid] = n
    nums[mid+1:end] = right.copy()
    return mid

nums = [125, 8, 0, 65, 23, 88, 1]
quick_sort(nums, 0, len(nums))
print(nums)