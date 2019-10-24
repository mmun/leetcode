#
# @lc app=leetcode id=26 lang=python3
#
# [26] Remove Duplicates from Sorted Array
#

# @lc code=start
class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        if not nums: return 0
        i = 1
        for j in range(1, len(nums)):
            if nums[j-1] != nums[j]:
                nums[i] = nums[j]
                i += 1
        return i
# @lc code=end
