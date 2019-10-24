#
# @lc app=leetcode id=9 lang=python3
#
# [9] Palindrome Number
#

# @lc code=start
class Solution:
    def isPalindrome(self, x: int) -> bool:
        y = 0
        i = x
        while i > 0:
            y *= 10
            y += i % 10
            i //= 10
        return x >= 0 and x == y

    # def isPalindrome(self, x: int) -> bool:
    #     s = str(x)
    #     n = len(s)
    #     for i in range(0, n // 2):
    #         if s[i] != s[n-1-i]:
    #             return False
    #     return True
# @lc code=end

