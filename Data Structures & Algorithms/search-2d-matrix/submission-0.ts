class Solution {
    /**
     * @param {number[][]} matrix
     * @param {number} target
     * @return {boolean}
     */
    searchMatrix(matrix: number[][], target: number): boolean {
        let res = matrix.find((arr)=>{
            if (arr[0] > target || arr[arr.length - 1] < target) {
                return false
            }

            return search(0, arr.length - 1, target, arr)
        })
        return res !== undefined
    }
}
function search(start:number,end: number, target: number, arr: number[]) {
    if(start > end) {
        return false;
    }

    let mid = Math.floor(end - (end - start) /2);
    let curr = arr[mid]
        console.log(curr, mid)

    if(target === curr) {
        return true
    }
    if (target > curr) {
        return search(mid + 1, end, target,arr)
    }
    if(target < curr) {
        return search(start, mid - 1, target,arr)
    }
}