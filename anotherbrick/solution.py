h, w, _ = map(int, input().split())

bricks = list(map(int, input().split()))

solutions = 0

current_width = w
for b in bricks:
	current_width -= b

	if current_width == 0:
		solutions += 1
		current_width = w

if current_width >= h:
	print("YES")
else:
	print("NO")
