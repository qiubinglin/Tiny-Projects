"""
https://www.hackerrank.com/challenges/abbr/problem
You can perform the following operation on some string, :

1. Capitalize zero or more of 's lowercase letters at some index i
   (i.e., make them uppercase).
2. Delete all of the remaining lowercase letters in .

Example:
a=daBcd and b="ABC"
daBcd -> capitalize a and c(dABCd) -> remove d (ABC)
"""

def abbr(a: str, b: str) -> bool:
    m = len(a)
    n = len(b)
    dp = [[False]*(n+1)]*(m+1)

    for i in range(m+1):
        dp[i][0] = True

    for i in range(1, m+1):
        for j in range(1, n+1):
            if a[i-1].isupper() and dp[i-1][j-1] and a[i-1] == b[j-1]:
                dp[i][j] = True
            elif a[i-1].upper() == b[j-1] and (dp[i-1][j-1] or dp[i-1][j]):
                dp[i][j] = True
            elif dp[i-1][j]:
                dp[i][j] = True
    return dp[m][n]

a = "daBcd"
b = "ABC"
if abbr(a, b):
    print("yes")
else:
    print("no")   