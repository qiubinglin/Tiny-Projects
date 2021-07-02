'''
    radix sort
'''

def radix_sort(nums, n):
    k = 0
    while k < n:
        ndict = [[] for _ in range(10)]
        drhs = 10**k
        for i in range(len(nums)):
            ndict[(nums[i]//drhs)%10].append(nums[i])
        idx = 0
        for i in range(len(ndict)):
            nums[idx:idx+len(ndict[i])] = ndict[i]
            idx += len(ndict[i])
        ndict.clear()
        k += 1

nums = [125, 8, 0, 65, 23, 88, 1]
radix_sort(nums, 3)
print(nums)