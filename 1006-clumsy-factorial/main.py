class Solution:
    def clumsy(self, n):
        op = "*"
        s = str(n)
        for i in range(n - 1, 0, -1):
            s = s + op + str(i)
            op = self.next_operator(op)
        print(s)
        return eval(s)

    def next_operator(self, current: str) -> str:
        match current:
            case "*":
                return "//"
            case "//":
                return "+"
            case "+":
                return "-"
            case "-":
                return "*"
        return "*"


Solution().clumsy(4)
