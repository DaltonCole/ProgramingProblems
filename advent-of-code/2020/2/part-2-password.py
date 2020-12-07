from sys import stdin

count = 0

for line in stdin:
    # Split by space
    repeat, char, password = line.split(' ')

    # Get repeat boundaries
    rmin, rmax = map(int, repeat.split('-'))
    # Get character to repeat
    char = char[0]

    first = True if rmin <= len(password) and password[rmin-1] == char else False
    second = True if rmin <= len(password) and password[rmax-1] == char else False

    if first != second and (first == True  or second == True):
        count += 1

print(count)
