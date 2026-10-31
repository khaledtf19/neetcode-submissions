class Solution {
    /**
     * @param {number[]} heights
     * @return {number}
     */
    maxArea(heights: number[]): number {
        let max = 0;
        let i = 0;
        let j = heights.length - 1;

        while (i < j) {
            let l = heights[i];
            let r = heights[j];

            let w = j - i;

            let min = Math.min(l,r);

            let res = min * w;

            if(res > max) {
                max = res;
            }

            if (l > r) {
                j--
            }else if (r > l) {
                i++
            }else {
                i++
                j--
            }

        }
        return max
    }
}
