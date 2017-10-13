import string
import random

s = ''

for i in range(200000):
	s += random.choice(string.ascii_letters)

with open('big', 'w') as f:
	f.write(str(200000) + '\n')
	f.write(s.lower())
