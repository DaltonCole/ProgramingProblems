import sys

i = 0
for line in sys.stdin:
	i += 1
	line = line.split()[1:]
	line = list(map(int, line))

	ma = max(line)
	mi = min(line)
	ra = ma - mi

	print("Case " + str(i) + ": " + str(mi) + " " + str(ma) + " " + str(ra))