from sys import stdin

num = 0

for line in stdin:
    row = line[:7]
    col = line[7:10]

    row = row.replace('B', '1')
    row = row.replace('F', '0')
    col = col.replace('R', '1')
    col = col.replace('L', '0')

    row = int(row, 2)
    col = int(col, 2)

    total = (row * 8) + col
    num = max(total, num)

print(num)
