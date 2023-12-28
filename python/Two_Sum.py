nums = [2, 7, 11, 15]
target = 9

class Solution:
    def two_sum(self, nums: list[int], target: list[int]) -> list[int]:
        num_indices = {}
        for i, num in enumerate(nums):
            complement = target - num 
            if complement in num_indices:
                return [num_indices[complement], i]
            num_indices[num] = i
        return []

solution = Solution()
result = solution.two_sum(nums, target)
print(f"Indices dos números que somam {target}: {result}")
