def change_position(position, direction):
    direction = map_direction[direction]
    position[0] += direction[0]
    position[1] += direction[1]
    return position


def move(maze, position, direction, command):
    if command == 'R':
        direction = (direction + 1) % 4
    elif command == 'L':
        direction = (direction - 1) % 4
    elif command == 'B':
        direction = (direction + 2) % 4

    position = change_position(position, direction)
    maze[position[0]][position[1]] = '.'
    return direction


cases = int(input())

print(cases)

map_direction = {
    0: (-1, 0),  # North
    1: (0, 1),  # East
    2: (1, 0),  # South
    3: (0, -1)  # West
}

for _ in range(cases):
    # Get movements
    movements = input()

    # Create maze
    maze = []
    for _ in range(100):
        maze.append([])
        for _ in range(100):
            maze[-1].append('#')

    # Starting position
    position = [1, 0]  # Second row, first column
    # Direction facing
    direction = 1  # Start off facing East

    # Complete all movements
    for c in movements:
        direction = move(maze, position, direction, c)

    # Re-orientate maze so there is a wall at the top and bottom
    while '.' in maze[-1] or '.' in maze[0]:
        maze.insert(0, maze.pop())

    # Get maze dimensions
    height, width = 0, 0
    for i, h in enumerate(maze):
        for j, w in enumerate(h):
            if w == '.':
                height = max(height, i + 1)
                width = max(width, j + 1)

    print(f'{height + 1} {width + 1}')
    for i, h in enumerate(maze):
        if i > height:
            break
        for j, w in enumerate(h):
            if j > width:
                break
            print(w, end='')
        print()
