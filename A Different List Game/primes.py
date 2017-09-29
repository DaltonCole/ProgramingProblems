def primes(n):
    primfac = []
    d = 2
    while d*d <= n:
        while (n % d) == 0:
            primfac.append(d)
            n //= d
        d += 1
    if n > 1:
       primfac.append(n)
    return primfac

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