class Solution {
    /**
     * @param {number[]} nums
     * @param {number} target
     * @return {number}
     */
    search(nums: number[], target: number): number {
        let start = 0;
        let end = nums.length -1;
        return binarySearch(start, end, target, nums)
    }
}

function binarySearch(start:number, end:number, target:number, arr: number[]){
     if(start > end) {
        return -1
    }
    let mid = Math.floor(start + (end - start) / 2);
    const curr = arr[mid];
    if(curr === target){
        return mid;
    }
   
    if (curr > target) {
        return binarySearch(start, mid - 1,target,arr);
    } 
     if (curr < target) {
        return binarySearch(mid + 1, end,target,arr);
    } 
}
