class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        answer = ''
        strs.sort()
        for i in range(len(strs[0])):
            curr = strs[0][i]
            if self.includesCharacter(strs,curr,i):
                answer += curr
            else:
                break
        return answer
            
    def includesCharacter(self, strs, curr, i):
        for x in strs[1:]:
            if x[i] == curr:
                continue
            else:
            	return False
        return True