#
# @lc app=leetcode id=38 lang=python3
#
# [38] Count and Say
#

# @lc code=start
class Solution:
    def countAndSay(self, n: int) -> str:
        ans = "1"
        for _ in range(n-1):
            next_ans = ""
            for digit, group in itertools.groupby(ans):
                print(group)
                next_ans += str(len(list(group))) + str(digit)
            ans = next_ans
        return ans        
# @lc code=end
