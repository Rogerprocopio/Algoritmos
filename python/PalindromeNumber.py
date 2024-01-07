class Solution:
    def isPalindrome(self, x: int) -> bool:
        str_x = str(x)
        return str_x == str_x[::-1] 
'''#str_x[::-1] é uma técnica de "slicing" 
    Quando utilizamos [::-1], o primeiro : indica o início do "slicing", o segundo : indica o final, e o valor -1 é o passo ou intervalo de iteração.
    Quando o valor do passo é negativo (-1 neste caso), isso significa que a string será percorrida de trás para frente, invertendo-a. '''

num = 121
resultado = print(Solution().isPalindrome(num))
