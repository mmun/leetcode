/*
 * @lc app=leetcode id=1026 lang=cpp
 *
 * [1026] Maximum Difference Between Node and Ancestor
 */
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    int maxAncestorDiff(TreeNode* root) {
        return dfs(root, 1000000, 0);
    }

    int dfs(TreeNode* root, int val_min, int val_max) {
        if (root) {
            int new_val_min = min(root->val, val_min);
            int new_val_max = max(root->val, val_max);
            return max(
                dfs(root->left, new_val_min, new_val_max),
                dfs(root->right, new_val_min, new_val_max)
            );
        } else {
            return val_max - val_min;
        }
    }
};

