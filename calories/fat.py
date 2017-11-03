calories_to_grams = [1/9, 1/4, 1/4, 1/4, 1/7]

# Not accurate enough
def convert_to_grams(food):
	grams = []
	percent = 0
	# No more C, just grams and %s
	for i, f in enumerate(food):
		if 'C' in f:
			grams.append(float(f.replace('C', '')) * calories_to_grams[i])
		elif '%' in f: # Add up all %s
			percent += float(f.replace('%', ''))
			grams.append(f)
		else:
			grams.append(float(f.replace('g', '')))

	# Find total grams given grams and percent
	total_grams = 0
	for f in grams:
		if type(f) == float:
			total_grams += f

	if percent != 0:
		total_grams = (total_grams * 100) / (100 - percent)

	for i, f in enumerate(food):
		if '%' in f:
			grams[i] = float(f.replace('%', '')) * total_grams / 100

	return grams, total_grams

grams_to_cal = [9,4,4,4,7]

def convert_to_cal(food):
	cal = []
	percent = 0
	total_cal = 0
	# No more g, just C and %s
	for i, f in enumerate(food):
		if f[-1] == 'C':
			num = float(f[:-1])
			cal.append(num)
			total_cal += num
		elif f[-1] == '%': # Add up all %s
			percent += float(f.replace('%', ''))
			cal.append(f)
		else:
			num = float(f[:-1]) * grams_to_cal[i]
			cal.append(num)
			total_cal += num

	if percent != 0:
		total_cal = (total_cal * 100) / (100 - percent)

	for i, f in enumerate(food):
		if f[-1] == '%':
			cal[i] = float(f[:-1]) * total_cal / 100

	return cal, total_cal

# fat, protein, sugar, starch and alcohol
while True:
	line = input()
	if line == '-':
		break;

	food = line.split()

	grams, total_grams = convert_to_cal(food)
	
	while True:
		line = input()
		if line == '-':
			break;
		food = line.split()
		more_grams, more_total = convert_to_cal(food)

		for i, g in enumerate(more_grams):
			grams[i] += g
		total_grams += more_total

	print(str(round((grams[0] / total_grams) * 100)) + "%")
