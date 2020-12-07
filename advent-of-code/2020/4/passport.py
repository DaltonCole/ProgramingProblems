from sys import stdin

count = 0
need = set(['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']) #, 'cid'])
current = set()

for line in stdin:
    if line == '\n':
        if len(need.difference(current)) == 0:
            count += 1
        current = set()
    else:
        line = line.rstrip()
        # Split line by space
        line = line.split(' ')
        for field in line:
            # Split field by :
            key, value = field.split(':')
            if key == 'byr':
                value = int(value)
                if not 1920 <= value <= 2002:
                    continue
            if key == 'iyr':
                value = int(value)
                if not 2010 <= value <= 2020:
                    continue
            if key == 'eyr':
                value = int(value)
                if not 2020 <= value <= 2030:
                    continue
            if key == 'hgt':
                if value[-2:] == 'cm':
                    value = int(value[:-2])
                    if not 150 <= value <= 193:
                        continue
                elif value[-2:] == 'in':
                    value = int(value[:-2])
                    if not 59 <= value <= 76:
                        continue
                else:
                    continue
            if key == 'hcl':
                if value[0] != '#' and len(value) != 7:
                    continue
                for i in range(1, 7):
                    if value[i] not in ['1', '2', '3', '4', '5', '6', '7', '8', '9' ,'0', 'a,','b','c','d','e','f']:
                        continue
            if key == 'ecl':
                if value not in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']:
                    continue
            if key == 'pid':
                if len(value) != 9:
                    continue
                for c in value:
                    if c not in ['0','1', '2', '3', '4', '5', '6', '7', '8', '9']:
                        continue
            current.add(key)

if current == need:
    count += 1

print(count)
