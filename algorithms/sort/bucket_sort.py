'''
    bucket sort
'''

def bucket_sort(nums: list, bucket_size: int = 5):
    if len(nums) == 0:
        raise Exception("no element")

    max_val, min_val = max(nums), min(nums)
    bucket_count = int((max_val - min_val) // bucket_size) + 1
    buckets = [ [] for _ in range(bucket_count) ]

    for i in range(len(nums)):
        buckets[int((nums[i] - min_val)//bucket_size)].append(nums[i])
    
    ret = []
    for i in range(len(buckets)):
        for j in range(len(buckets[i])):
            ret.append(buckets[i][j])
    return sorted(ret)

nums = [125, 8, 0, 65, 23, 88, 1]
print(bucket_sort(nums))