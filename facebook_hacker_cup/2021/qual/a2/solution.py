from typing import Dict, List, Tuple, Set
import string
from prettytable import PrettyTable


uppercase = string.ascii_uppercase


def update_table(table: Dict[str, Dict[str, int]], update: List[Tuple[str, str]]) -> None:
    if len(update) == 0:
        return

    to_update = []
    for x, y in update:
        for col in uppercase:
            if col == x or col == y:
                continue
            # There was a way to get from col to x
            if table[col][x] != -1:
                # See if going from col to y through x is better
                if table[col][y] > table[x][y] + table[col][x] or table[col][y] == -1:
                    table[col][y] = table[x][y] + table[col][x]
                    # Explore if going through col to get to y for new location
                    # is better
                    to_update.append((col, y))

    update_table(table, to_update)


def common_letters(table, letters) -> Set[str]:
    common = []
    for letter in letters:
        tmp = set()
        for key, value in table[letter].items():
            if value != -1:
                tmp.add(key)
        common.append(tmp)
    return set.intersection(*common)


def print_table(table) -> None:
    ptable = PrettyTable()
    ptable.field_names = [''] + list(uppercase)

    for letter, row in table.items():
        ptable.add_row([letter] + list(row.values()))

    print(ptable)


if __name__ == '__main__':
    cases = int(input())

    for case in range(cases):
        word = input()
        letters = {}
        for letter in word:
            if letter not in letters:
                letters[letter] = 0
            letters[letter] += 1

        num_rules = int(input())
        rules = []
        for _ in range(num_rules):
            x, y = input()
            rules.append((x, y))

        table = {letter: {let: -1 for let in uppercase} for letter in uppercase}

        # Self
        for letter in table.keys():
            table[letter][letter] = 0

        # Rules
        for x, y in rules:
            table[x][y] = 1

        # Update table with all path distances
        update_table(table, rules)

        # print_table(table)

        common = common_letters(table, list(letters.keys()))
        # print(len(common))

        distance = -1

        for com in common:
            total_dist = 0
            for letter, value in letters.items():
                total_dist += (value * table[letter][com])

            distance = min(distance, total_dist) if distance != -1 else total_dist

        print(f'Case #{case + 1}: {distance}')
