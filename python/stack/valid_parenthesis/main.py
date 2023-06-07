class Solution:
    pair_dict = {"(": ")", "{": "}", "[": "]"}
    def isValid(self, s: str) -> bool:
        stack = []
        for c in s:
            stack.append(c)
            if len(stack) >= 2:
                if stack[-2] in self.pair_dict and self.pair_dict[stack[-2]] == stack[-1]:
                    stack.pop()
                    stack.pop()

        if len(stack) == 0:
            return True

        return False       