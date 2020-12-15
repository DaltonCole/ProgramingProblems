from sys import stdin

mask = ''
mem = {}
key = 0
value = 0

def change_bit(key):
    global value
    try:
        index = key.index('X')
        for bit in ['0', '1']:
            key[index] = bit
            change_bit(key.copy())
    except:
        tmp = ''.join(key)
        mem[int(tmp, 2)] = value

for value in stdin:
    if 'mask' in value:
        mask = value.split('=')[1].strip()
    else:
        # Get key and value
        value = value[4:]
        key, value = value.split(']')
        key = int(key)
        value = int(value.split('=')[1].strip())
        # Convert key to binary string
        key = f'{key:#b}'[2:]
        # Add leading zeros
        key = ('0' * (36 - len(key))) + key
        key = [x for x in key]
        # Add '1' and 'x' from mask to value
        for i in range(len(key)):
            if mask[i] != '0':
                key[i] = mask[i]
        change_bit(key)

total = 0
for key, value in mem.items():
    total += value

print(total)
