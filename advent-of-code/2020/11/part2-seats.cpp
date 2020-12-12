#include <iostream>
#include <vector>

using namespace std;

const char OCCUPIED = '#';
const char EMPTY = 'L';
const char FLOOR = '.';

int adjacent_occupied_seats(const vector<string>& seats, const uint i, const uint j) {
    int count = 0;
    // Up
    for(int x = i-1; x >= 0; x--) {
        if(seats[x][j] == OCCUPIED) {
            count++;
            break;
        }
        if(seats[x][j] == EMPTY) {
            break;
        }
    }
    // Down
    for(int x = i+1; x < seats.size(); x++) {
        if(seats[x][j] == OCCUPIED) {
            count++;
            break;
        }
        if(seats[x][j] == EMPTY) {
            break;
        }
    }
    // Left
    for(int y = j-1; y >= 0; y--) {
        if(seats[i][y] == OCCUPIED) {
            count++;
            break;
        }
        if(seats[i][y] == EMPTY) {
            break;
        }
    }
    // Right
    for(int y = j+1; y < seats[i].size(); y++) {
        if(seats[i][y] == OCCUPIED) {
            count++;
            break;
        }
        if(seats[i][y] == EMPTY) {
            break;
        }
    }
    // Up-Left
    for(int x = i-1, y = j-1; x >= 0 && y >= 0; x--, y--) {
        if(seats[x][y] == OCCUPIED) {
            count++;
            break;
        }
        if(seats[x][y] == EMPTY) {
            break;
        }
    }
    // Up-Right
    for(int x = i-1, y = j+1; x >= 0 && y < seats[x].size(); x--, y++) {
        if(seats[x][y] == OCCUPIED) {
            count++;
            break;
        }
        if(seats[x][y] == EMPTY) {
            break;
        }
    }
    // Down-Left
    for(int x = i+1, y = j-1; x < seats.size() && y >= 0; x++, y--) {
        if(seats[x][y] == OCCUPIED) {
            count++;
            break;
        }
        if(seats[x][y] == EMPTY) {
            break;
        }
    }
    // Down-Right
    for(int x = i+1, y = j+1; x < seats.size() && y < seats[x].size(); x++, y++) {
        if(seats[x][y] == OCCUPIED) {
            count++;
            break;
        }
        if(seats[x][y] == EMPTY) {
            break;
        }
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
                if(old_seats[i][j] == OCCUPIED && occupied_seats >= 5) {
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
