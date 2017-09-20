number_list = []

for i in range(9):
	number_list.append(int(input()))

for a in number_list:
	for b in number_list:
		if b == a:
			continue
		for c in number_list:
			if c == a or c == b:
				continue
			for d in number_list:
				if d == a or d == b or d == c:
					continue
				for e in number_list:
					if e == d or e == c or e == b or e == a:
						continue
					for f in number_list:
						if f == a or f == b or f == c or f == d or f == e:
							continue
						for g in number_list:
							if g == a or g == b or g == c or g == d or g == e or g == f:
								continue
							if a + b + c + d + e + f + g == 100:
								print(a)
								print(b)
								print(c)
								print(d)
								print(e)
								print(f)
								print(g)
								quit()
