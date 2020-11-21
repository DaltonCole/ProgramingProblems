#include <iostream>
using namespace std;

const int MAX_COUNTRIES = 50;

int main() {
    int cases;
    int countries;
    char in[MAX_COUNTRIES];
    char out[MAX_COUNTRIES];
    char travel[MAX_COUNTRIES][MAX_COUNTRIES];

    cin >> cases;

    for(int i = 0; i < cases; i++) {
        cin >> countries;
        for(int j = 0; j < countries; j++) {
            cin >> in[j];
        }
        for(int j = 0; j < countries; j++) {
            cin >> out[j];
        }
        // Make all cases 'N' except for the diagonal
        for(int y = 0; y < countries; y++) {
            for(int x = 0; x < countries; x++) {
                if(y == x) {
                    travel[y][x] = 'Y';
                }
                else {
                    travel[y][x] = 'N';            
                }
            }       
        }
        // Set sub-diagonal to 'Y' if in and out match
        for(int a = 0; a < countries - 1; a++) {
            // Lower diagonal: Current in and Next out
            if(in[a] == 'Y' && out[a+1] == 'Y') {
                travel[a+1][a] = 'Y';
            }
            // Upper diagonal: Next in and Current out
            if(in[a+1] == 'Y' && out[a] == 'Y') {
                travel[a][a+1] = 'Y';
            }
        }
        // Lower half of matrix
        for(int y = 2; y < countries; y++) {
            for(int x = y - 2; x >= 0; x--) {
                if(travel[y-1][x] == 'Y' && travel[y][x+1] == 'Y') {
                    travel[y][x] = 'Y';
                }
            }
        }
        // Upper half of matrix
        for(int y = countries - 3; y >= 0; y--) {
            for(int x = y + 2; x < countries; x++) {
                if(travel[y][x-1] == 'Y' && travel[y+1][x] == 'Y') {
                    travel[y][x] = 'Y';
                }
            }
        }

        // Print
        cout << "Case #" << (i + 1) << ":" << endl;
        for(int y = 0; y < countries; y++) {
            for(int x = 0; x < countries; x++) {
                cout << travel[y][x];
            }
            cout << endl;
        }
    }
    return 0;
}
