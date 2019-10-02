/*
 * @lc app=leetcode id=200 lang=javascript
 *
 * [200] Number of Islands
 */
/**
 * @param {character[][]} grid
 * @return {number}
 */
var numIslands = function(grid) {
    let m = grid.length;
    let n = grid.length && grid[0].length;
    let dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];

    let visit = (i, j) => {
        grid[i][j] = 'V';
        for ([di, dj] of dirs) {
            if (i+di >= 0 && i+di < m && j+dj >= 0 && j+dj < n) {
                if (grid[i+di][j+dj] === '1') {
                    visit(i+di, j+dj);
                }
            }
        }
    };

    let count = 0;

    for (let i = 0; i < m; i++) {
        for (let j = 0; j < n; j++) {
            if (grid[i][j] === '1') {
                count++;
                visit(i, j);
            }
        }
    }

    return count;
};

