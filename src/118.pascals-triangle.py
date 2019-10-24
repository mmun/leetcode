#
# @lc app=leetcode id=118 lang=python
#
# [118] Pascal's Triangle
#

# @lc code=start
class Solution(object):
    def generate(self, numRows):
        """
        :type numRows: int
        :rtype: List[List[int]]
        """
        rows = []

        for i in range(0, numRows):
            rows.append([0] * (i+1))
            rows[i][0] = 1
            rows[i][i] = 1

            for j in range(1, i):
                rows[i][j] = rows[i-1][j-1] + rows[i-1][j]
        
        return rows
# @lc code=end
