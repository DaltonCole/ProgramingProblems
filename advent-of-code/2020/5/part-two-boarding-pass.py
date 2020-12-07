from sys import stdin

seats = set()

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
    seats.add(total)

seats = sorted(seats)
for i in range(1, len(seats)):
    if seats[i-1] + 2 == seats[i]:
        print(seats[i-1] + 1)
