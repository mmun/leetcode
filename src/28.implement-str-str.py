#
# @lc app=leetcode id=28 lang=python3
#
# [28] Implement strStr()
#

# @lc code=start
# Knuth-Morris-Pratt, for fun.
class Solution:

    def strStr(self, s: str, p: str) -> int:
        if not p: return 0

        π = [0] * len(p)
        j = 0
        for i in range(1, len(p)):
            while j > 0 and p[i] != p[j]: j = π[j-1]
            if p[i] == p[j]: j += 1
            π[i] = j

        j = 0
        for i in range(len(s)):
            while j > 0 and s[i] != p[j]: j = π[j-1]
            if s[i] == p[j]: j += 1
            if j == len(p): return i - (j-1)

        return -1
# @lc code=end
