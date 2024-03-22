class Solution:
    def plusOne(self, digits: List[int]) -> List[int]:
        # Convert the list of digits to a string representation
        s = ''.join(str(x) for x in digits)
        
      
        s = str(int(s) + 1)
        
     
        result = [int(digit) for digit in s]
        
        return result