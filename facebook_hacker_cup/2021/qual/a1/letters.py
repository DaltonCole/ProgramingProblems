
vowels = set(['A', 'E', 'I', 'O', 'U'])
MAX_VALUE = 9999999


def sub_solution(letters: dict, num_vowels: int, num_const: int) -> int:
    best = MAX_VALUE
    for current_letter, num_letter in letters.items():
        num_letter = letters[current_letter]

        if current_letter in vowels:
            cost = (2 * (num_vowels - num_letter)) + num_const
        else:
            cost = (2 * (num_const - num_letter)) + num_vowels

        best = min(best, cost)

    return best


def all_same(letters: dict, num_vowels: int, num_const: int) -> int:
    if num_vowels == 0:
        return num_const
    if num_const == 0:
        return num_vowels
    return MAX_VALUE


if __name__ == '__main__':
    cases = int(input())

    for case in range(cases):
        letters = {}
        num_vowels = 0
        num_const = 0

        word = input()

        for letter in word:
            if letter not in letters:
                letters[letter] = 0
            letters[letter] += 1

            if letter in vowels:
                num_vowels += 1
            else:
                num_const += 1

        answer = min(sub_solution(letters, num_vowels, num_const),
                     all_same(letters, num_vowels, num_const))

        print(f'Case #{case + 1}: {answer}')
