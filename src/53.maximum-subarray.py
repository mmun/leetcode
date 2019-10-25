#
# @lc app=leetcode id=53 lang=python3
#
# [53] Maximum Subarray
#

# @lc code=start
class Solution:
    def maxSubArray(self, a: List[int]) -> int:
        n = len(a)
        dp = [0] * n
        dp[0] = a[0]

        for i in range(1, n):
            dp[i] = max(dp[i-1], 0) + a[i]
        
        return max(dp)
# @lc code=end
