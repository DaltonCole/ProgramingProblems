# This video helped a lot: https://www.youtube.com/watch?v=zIFehsBHB8o&ab_channel=MathswithJay
def chinese_remainder_theorem(congruent):
    # Get the mods
    mod = [x[0] for x in congruent]
    # Get the remainders
    bi = [x[1] for x in congruent]
    # Get all of the mods multiplied together
    N = 1
    for n, b in congruent:
        N *= n
    # Get the mods multiplied together without the ith mod
    Ni = [int(round(N / x[0])) for x in congruent]
    # Find the inverse of Ni
    xi = []
    for i in range(len(congruent)):
        coeff = Ni[i] - (int(Ni[i] / mod[i]) * mod[i])
        for c in range(1, mod[i]):
            if (coeff * c) % mod[i] == mod[i]-1:
                xi.append(c)
                break
    # Multiply all columns together
    bnx = [a*b*c for a, b, c in zip(bi, Ni, xi)]
    # Add all rows together
    total = sum(bnx)

    # Sanity check
    print(f'bi: {bi}')
    print(f'Ni: {Ni}')
    print(f'xi: {xi}')

    # Return the first result
    return total % N

_ = input()

sched = input().split(',')

time = []
offset = []
for i, x in enumerate(sched):
    if x != 'x':
        time.append(int(x))
        offset.append(i)

time = list(zip(time, offset))
print(chinese_remainder_theorem(time))
