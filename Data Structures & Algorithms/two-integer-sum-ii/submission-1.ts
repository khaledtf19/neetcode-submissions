class Solution {
    /**
     * @param {number[]} numbers
     * @param {number} target
     * @return {number[]}
     */
    twoSum(numbers: number[], target: number): number[] {
        let start = 0; 
        let end = numbers.length - 1;
        while(start < end) {
            if(numbers[end] + numbers[start] > target) {
                end--;
            }else if (numbers[end] + numbers[start] < target) {
                start++
            }else {
                break
            }
        }
        return [start+1, end + 1]

    }
}
