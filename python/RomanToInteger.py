class Solution: 
    def convertToInt(self,str:str) -> int:
        roman = {
            'I': 1,
            'V': 5,
            'X': 10,
            'L': 50,
            'C': 100,
            'D': 500,
            'M': 1000,
        }
        total = 0
        prev_value = 0

        for char in str:
            current_value = roman[char]
            if current_value > prev_value:
                total += current_value -2 * prev_value
            else:
                total += current_value
                prev_value = current_value
        return total 
        
roman_numeral = 'III'
init = Solution()
result = init.convertToInt(roman_numeral)
print(f"The integer value of {roman_numeral} is: {result}")