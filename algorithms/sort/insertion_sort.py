'''
    insertion sort
'''

def insertion_sort(nums):
    if len(nums) <= 1:
        return
    for i in range(1, len(nums)):
        for j in range(i, 0, -1):
            if nums[j] < nums[j-1]:
                nums[j], nums[j-1] = nums[j-1], nums[j]
            else:
                break

nums = [125, 8, 0, 65, 23, 88, 1]
insertion_sort(nums)
print(nums)