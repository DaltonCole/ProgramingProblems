#include <iostream>
#include <cmath>

using namespace std;

const char NORTH = 'N';
const char SOUTH = 'S';
const char WEST = 'W';
const char EAST = 'E';
const char LEFT = 'L';
const char RIGHT = 'R';
const char FORWARD = 'F';

char rotate(const char dir, int mag, const char rotate_dir) {
    if(mag == 0) {
        return dir;
    }

    // 180 degrees
    if(mag == 180) {
        if(dir == NORTH) {
            return SOUTH;
        }
        if(dir == EAST) {
            return WEST;
        }
        if(dir == SOUTH) {
            return NORTH;
        }
        if(dir == WEST) {
            return EAST;
        }
    }

    if(rotate_dir == LEFT) {
        mag = (mag + 180) % 360;
    }
    // 90 degrees
    if(mag == 90) {
        if(dir == NORTH) {
            return EAST;
        }
        if(dir == EAST) {
            return SOUTH;
        }
        if(dir == SOUTH) {
            return WEST;
        }
        if(dir == WEST) {
            return NORTH;
        }
    }
    
    // 270 degrees
    if(mag == 270) {
        if(dir == NORTH) {
            return WEST;
        }
        if(dir == EAST) {
            return NORTH;
        }
        if(dir == SOUTH) {
            return EAST;
        }
        if(dir == WEST) {
            return SOUTH;
        }
    }
    return dir;
}

int main() {
    char dir = EAST;
    int x = 0;
    int y = 0;
    char inst;
    int mag;

    while(cin >> inst >> mag) {
        cout << inst << mag << endl;
        if(inst == NORTH || (inst == FORWARD && dir == NORTH)) {
            x += mag;
        } else if(inst == SOUTH || (inst == FORWARD && dir == SOUTH)) {
            x -= mag;
        } else if(inst == EAST || (inst == FORWARD && dir == EAST)) {
            y += mag;
        } else if(inst == WEST || (inst == FORWARD && dir == WEST)) {
            y -= mag;
        } else if(inst == RIGHT || inst == LEFT) {
            dir = rotate(dir, mag, inst);
        } else {
            cout << "ERROR" << endl;
        }
    }

    cout << (abs(x) + abs(y)) << endl;

    return 0;
}
