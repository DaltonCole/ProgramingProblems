import math

while True:
    max_dist, num_hives = map(float, input().split())
    num_hives = int(num_hives)

    if num_hives == 0:
        break

    hives = []
    for _ in range(num_hives):
        x, y = map(float, input().split())
        hives.append((x, y))

    sweet, sour = 0, 0
    sweet = [1 for _ in range(num_hives)]
    for i in range(len(hives)):
        for j in range(i + 1, len(hives)):
            dist = math.sqrt(pow(hives[i][0] - hives[j][0], 2) + pow(hives[i][1] - hives[j][1], 2))
            if dist <= max_dist:
                sweet[i] = 0
                sweet[j] = 0

    sour = sweet.count(0)
    sweet = len(sweet) - sour

    print(f'{sour} sour, {sweet} sweet')
