from main import Solution

class TestSolution:
    def test_one(self):
        assert(Solution().isValid("()") == True)
    
    def test_two(self):
        assert(Solution().isValid("()[]{}") == True)
    
    def test_three(self):
        assert(Solution().isValid("(]") == False)