'''
    counting sort
'''

def counting_sort(nums):
    nmin = min(nums)
    nmax = max(nums)
    ndict = [0]*(nmax+1)
    for i in range(len(nums)):
        ndict[nums[i]] += 1

    idx = 0
    for i in range(nmin, nmax+1):
        while ndict[i] > 0:
            nums[idx] = i
            idx += 1
            ndict[i] -= 1

nums = [125, 8, 0, 65, 23, 88, 1]
counting_sort(nums)
print(nums)
