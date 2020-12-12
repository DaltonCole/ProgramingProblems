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

int main() {
    int x = 0;
    int y = 0;
    int wy = 1;
    int wx = 10;
    int tmp;
    char inst;
    int mag;

    while(cin >> inst >> mag) {
        cout << inst << mag << endl;
        if(inst == NORTH) {
            wy += mag;
        } else if(inst == SOUTH)  {
            wy -= mag;
        } else if(inst == EAST) {
            wx += mag;
        } else if(inst == WEST) {
            wx -= mag;
        } else if(inst == RIGHT || inst == LEFT) {
            // Rotate 90 degrees clockwise
            if((inst == RIGHT && mag == 90) || (inst == LEFT && mag == 270)) {
                tmp = wx;
                wx = wy;
                wy = tmp * -1;
            }
            // Rotate 180 degrees
            if(mag == 180) {
                wx *= -1;
                wy *= -1;
            }
            // Rotate 90 degrees counter-clockwise
            if((inst == RIGHT && mag == 270) || (inst == LEFT && mag == 90)) {
                tmp = wx;
                wx = wy * -1;
                wy = tmp;
            }
        } else if(inst == FORWARD) {
            cout << "forward: " << wx << wy << endl;
            for(int i = 0; i < mag; i++) {
                x += wx;
                y += wy;
            }
        } else {
            cout << "ERROR" << endl;
        }
        cout << (x) << " " << (y) << endl;
    }

    cout << abs(x) << " " << abs(y) << endl;

    cout << (abs(x) + abs(y)) << endl;

    return 0;
}
