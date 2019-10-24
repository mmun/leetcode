#
# @lc app=leetcode id=14 lang=python3
#
# [14] Longest Common Prefix
#

# @lc code=start
class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        if not strs:
            return ""

        shortest = min(strs, key=len)
        for i, c in enumerate(shortest):
            if any(s[i] != c for s in strs):
                return shortest[:i]
        return shortest
# @lc cod   e=end

