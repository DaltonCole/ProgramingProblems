first, second = input().split()

# Reverse
first = first[::-1]
second = second[::-1]

first = int(first)
second = int(second)

if first > second:
	print(first)
else:
	print(second)