class Solution:
    def concatenatedBinary(self, n: int) -> int:
        s = ''
        for i in range(1, n+1):
            s = s + '{0:b}'.format(i)
        s = int(s, base=2) % (1000000007)
        return s


if __name__ == '__main__':
    a = Solution()
    assert a.concatenatedBinary(1) == 1
    assert a.concatenatedBinary(3) == 27
    assert a.concatenatedBinary(12) == 505379714
    a.concatenatedBinary(100000)
