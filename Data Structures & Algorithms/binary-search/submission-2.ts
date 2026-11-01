class Solution {
    /**
     * @param {number[]} nums
     * @param {number} target
     * @return {number}
     */
    search(nums: number[], target: number): number {
        let start = 0;
        let end = nums.length -1;
        while (start <= end) {
            const mid = Math.floor(start + (end - start) / 2);
            const curr = nums[mid];
            if (curr === target) {
                return mid;
            }else if (curr > target) {
                end = mid - 1
            }else {
                start = mid + 1
            }
        }
        return -1
    }
}
