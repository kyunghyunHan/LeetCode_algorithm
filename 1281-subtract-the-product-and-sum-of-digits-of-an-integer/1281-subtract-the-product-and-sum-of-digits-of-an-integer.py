class Solution:
    def subtractProductAndSum(self, n: int) -> int:
        s = str(n)
        nums = [int(x) for x in s]
        _sum = sum(nums)
        _product = 1
        for num in nums:
            _product *= num

        return _product - _sum
        