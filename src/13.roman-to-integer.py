#
# @lc app=leetcode id=13 lang=python3
#
# [13] Roman to Integer
#

# @lc code=start
values = {
    'I': 1,
    'V': 5,
    'X': 10,
    'L': 50,
    'C': 100,
    'D': 500,
    'M': 1000
}

class Solution:
    def romanToInt(self, s: str) -> int:
        n = values[s[-1]]
        for i in range(0, len(s)-1):
            if values[s[i]] < values[s[i+1]]:
                n -= values[s[i]]
            else:
                n += values[s[i]]
        return n

# @lc code=end
