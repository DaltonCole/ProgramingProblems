#include <iostream>
#include <vector>

using namespace std;

const char OCCUPIED = '#';
const char EMPTY = 'L';
const char FLOOR = '.';

int adjacent_occupied_seats(const vector<string>& seats, const uint i, const uint j) {
    int count = 0;
    bool up = i > 0;
    bool down = (i + 1 < seats.size());
    bool left = j > 0;
    bool right = (j + 1 < seats[i].size());
    // Up
    if(up && seats[i-1][j] == OCCUPIED) {
        count++;
    }
    // Down
    if(down && seats[i+1][j] == OCCUPIED) {
        count++;
    }
    // Left
    if(left && seats[i][j-1] == OCCUPIED) {
        count++;
    }
    // Right
    if(right && seats[i][j+1] == OCCUPIED) {
        count++;
    }
    // Up-Left
    if(up && left && seats[i-1][j-1] == OCCUPIED) {
        count++;
    }
    // Up-Right
    if(up && right && seats[i-1][j+1] == OCCUPIED) {
        count++;
    }
    // Down-Left
    if(down && left && seats[i+1][j-1] == OCCUPIED) {
        count++;
    }
    // Down-Right
    if(down && right && seats[i+1][j+1] == OCCUPIED) {
        count++;
    }

    return count;
}

void print(const vector<string>& seats) {
    for(const auto row : seats) {
        cout << row << endl;
    }
    cout << endl;
    return;
}

int main() {
    vector<string> seats;
    vector<string> old_seats;
    string tmp;
    int count = 0;
    int occupied_seats = 0;

    while(cin >> tmp) {
        seats.push_back(tmp);
    }

    do {
        // Increment count
        count++;
        // Make old equal to new configuration
        old_seats = seats;
        // For each row
        for(uint i = 0; i < seats.size(); i++) {
            // For each column
            for(uint j = 0; j < seats[i].size(); j++) {
                occupied_seats = adjacent_occupied_seats(old_seats, i, j);
                if(old_seats[i][j] == EMPTY && occupied_seats == 0) {
                    seats[i][j] = OCCUPIED;
                }
                if(old_seats[i][j] == OCCUPIED && occupied_seats >= 4) {
                    seats[i][j] = EMPTY;
                }
            }
        }
        //print(seats);
    } while(old_seats != seats);

    cout << count << endl;

    count = 0;
    for(const auto& row : seats) {
        for(const auto c : row) {
            if(c == OCCUPIED) {
                count++;
            }
        }
    }

    cout << count << endl;

    return 0;
}
