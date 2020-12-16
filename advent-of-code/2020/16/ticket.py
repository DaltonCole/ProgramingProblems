from sys import stdin

class Ticket:
    def __init__(self):
        self.fields = {}
        self.your_ticket = []
        self.nearby_tickets = []
        self.all_num = set()

    def read_in_input(self):
        step = 0
        for line in stdin:
            if 'ticket' in line:
                step += 1
                continue

            if len(line) < 2:
                continue

            if step == 0:
                word, rest = line.split(':')
                self.fields[word] = set()
                first, second = rest.split(' or ')
                a, b = first.split('-')
                x, y = second.split('-')

                for i in range(int(a), int(b) + 1):
                    self.fields[word].add(i)
                for i in range(int(x), int(y) + 1):
                    self.fields[word].add(i)

            if step == 1:
                for num in line.split(','):
                    self.your_ticket.append(int(num))

            if step == 2:
                self.nearby_tickets.append([])
                for num in line.split(','):
                    self.nearby_tickets[-1].append(int(num))

        # Add to all
        for key, value in self.fields.items():
            for num in value:
                self.all_num.add(num)

    def verify(self):
        count = 0
        remove_indexes = set()

        for i, ticket in enumerate(self.nearby_tickets):
            for num in ticket:
                if num not in self.all_num:
                    count += num
                    remove_indexes.add(i)

        print(f'Part 1: {count}')

        # Remove
        for index in sorted(list(remove_indexes), reverse=True):
            del self.nearby_tickets[index]

    def classify(self):
        classes = {key: set() for key in self.fields.keys()}

        columns = [[] for _ in range(len(self.nearby_tickets[0]))]
        for row in self.nearby_tickets:
            for i, num in enumerate(row):
                columns[i].append(num)

        for key, value in self.fields.items():
            for index, col in enumerate(columns):
                for num in col:
                    if num not in value:
                        break
                else:
                    classes[key].add(index)

        classes = {k: v for k, v in sorted(classes.items(), key=lambda x: len(x[1]))}

        remove = set()
        for key in classes.keys():
            classes[key] = classes[key] - remove
            remove = remove.union(classes[key])
            classes[key] = list(classes[key])[0]

        # print(classes)

        total = 1
        for key, value in classes.items():
            if 'departure' in key:
                total *= self.your_ticket[value]

        print(f'Part 2: {total}')


if __name__ == '__main__':
    ticket = Ticket()
    ticket.read_in_input()
    ticket.verify()
    ticket.classify()
