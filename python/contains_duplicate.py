class Solution:
    def containsDuplicate(self, nums: list[int]) -> bool:
        d = set()
        for i in nums:
            if i in d:
                return True
            else:
                d.add(i)
        return False
