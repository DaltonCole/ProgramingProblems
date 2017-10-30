from math import pi

cir = pi * 60 / 15
piece = (cir / 28)

c_to_n = {}
c_to_n['A'] = 0
c_to_n['B'] = 1
c_to_n['C'] = 2
c_to_n['D'] = 3
c_to_n['E'] = 4
c_to_n['F'] = 5
c_to_n['G'] = 6
c_to_n['H'] = 7
c_to_n['I'] = 8
c_to_n['J'] = 9
c_to_n['K'] = 10
c_to_n['L'] = 11
c_to_n['M'] = 12
c_to_n['N'] = 13
c_to_n['O'] = 14
c_to_n['P'] = 15
c_to_n['Q'] = 16
c_to_n['R'] = 17
c_to_n['S'] = 18
c_to_n['T'] = 19
c_to_n['U'] = 20
c_to_n['V'] = 21
c_to_n['W'] = 22
c_to_n['X'] = 23
c_to_n['Y'] = 24
c_to_n['Z'] = 25
c_to_n[' '] = 26
c_to_n['\''] = 27

def time(sentence):
	distance = 0
	for i in range(len(sentence) - 1):
		dif = abs(c_to_n[sentence[i]] - c_to_n[sentence[i+1]])
		#print(sentence[i] + " " + sentence[i+1] + " " +str(dif))
		distance += dif * piece
	time = distance + len(sentence)
	print(time)

cases = int(input())

for c in range(cases):
	time(input())