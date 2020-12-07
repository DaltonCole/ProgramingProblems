from sys import stdin

count = 0

for line in stdin:
    # Split by space
    repeat, char, password = line.split(' ')

    # Get repeat boundaries
    rmin, rmax = map(int, repeat.split('-'))
    # Get character to repeat
    char = char[0]

    if rmin <= password.count(char) <= rmax:
        count += 1

print(count)
