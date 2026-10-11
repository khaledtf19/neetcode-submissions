class Solution {
    /**
     * @param {string} s
     * @return {boolean}
     */
    isPalindrome(s: string): boolean {
        let str = s.split("").filter(isAlphanumeric).join();
        let res = true;
        for(let i = 0, j = s.length - 1; i < j; i++,j--){
            let right = s[i];
            let left = s[j];
            while(!isAlphanumeric(right) && i < j){
                i++
                right = s[i]
            }
            while(!isAlphanumeric(left) && j > i){
                j--
                left = s[j]
            }

            if (left.toLocaleLowerCase() !== right.toLocaleLowerCase()) {
                res = false;
                break;
            }  
        }
        return res
    }
}

function isAlphanumeric(char: string) {
  if (!char || char.length !== 1) return false;
  
  const code = char.charCodeAt(0);
  return (code > 47 && code < 58) ||   // numeric (0-9)
         (code > 64 && code < 91) ||   // upper alpha (A-Z)
         (code > 96 && code < 123);    // lower alpha (a-z)
}