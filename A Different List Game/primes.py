import itertools as it
def primes( ):
    D = { 9: 3, 25: 5 }
    yield 2
    yield 3
    yield 5
    MASK= 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0,
    MODULOS= frozenset( (1, 7, 11, 13, 17, 19, 23, 29) )

    for q in it.compress(
            it.islice(it.count(7), 0, None, 2),
            it.cycle(MASK)):
        p = D.pop(q, None)
        if p is None:
            D[q*q] = q
            yield q
        else:
            x = q + 2*p
            while x in D or (x%30) not in MODULOS:
                x += 2*p
            D[x] = p

def unique_list(numbers):
    while len(numbers) != len(set(numbers)):
        numbers.sort()
        print(numbers)
        for i in range(1, len(numbers)):
            if numbers[i-1] == numbers[i]:
                squared = numbers[i] ** 2
                if squared in numbers:
                    while(True):
                        if squared in numbers:
                            squared *= numbers[i]
                        else:
                            break
                    squared /= numbers[i]
                    index = numbers.index(squared)
                    numbers[index] *= numbers[i]
                    numbers.pop(i)
                else:
                    numbers.append(squared)
                    numbers.pop(i-1)
                    numbers.pop(i-1)
                break

    print(numbers)
    return len(numbers)

number = int(input())

prime = primes(number)

print(unique_list(prime))