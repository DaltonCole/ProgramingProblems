from sys import stdin

mask = ''
mem = {}

for value in stdin:
    if 'mask' in value:
        mask = value.split('=')[1].strip()
    else:
        # Get key and value
        value = value[4:]
        key, value = value.split(']')
        key = int(key)
        value = int(value.split('=')[1].strip())
        # Convert value to binary string
        value = f'{value:#b}'[2:]
        # Add leading zeros
        value = ('0' * (36 - len(value))) + value
        value = [x for x in value]
        # Add mask
        for i in range(len(value)):
            if mask[i] != 'X':
                value[i] = mask[i]
        value = ''.join(value)
        value = int(value, 2)
        mem[key] = value

total = 0
for key, value in mem.items():
    total += value

print(total)
