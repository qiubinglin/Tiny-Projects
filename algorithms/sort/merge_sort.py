'''
    merge sort
'''

def merge_sort(nums):
    if len(nums) == 0:
        return 0
    _merge_sort(nums, 0, len(nums)-1)

def _merge_sort(nums, beg, end):
    if beg >= end:
        return
    if beg + 1 == end:
        nums[beg], nums[end] = (nums[end], nums[beg]) if nums[beg] > nums[end] else (nums[beg], nums[end])
    else:
        mid = beg + (end-beg)//2
        _merge_sort(nums, beg, mid)
        _merge_sort(nums, mid+1, end)
        _merge(nums, beg, end, mid)

def _merge(nums, beg, end, mid):
    A = nums[beg:mid+1]
    B = nums[mid+1:end+1]

    i = j = 0
    while i < len(A) and j < len(B):
        if A[i] <= B[j]:
            nums[beg+i+j] = A[i]
            i += 1
        else:
            nums[beg+i+j] = B[j]
            j += 1
    while i < len(A):
        nums[beg+i+j] = A[i]
        i += 1
    while j < len(B):
        nums[beg+i+j] = B[j]
        j += 1

nums = [125, 8, 0, 65, 23, 88, 1]
merge_sort(nums)
print(nums)