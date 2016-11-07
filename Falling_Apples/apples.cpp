#include <iostream>
//#include <vector>
using namespace std;

int main() {
    int r = 0, c = 0;
    cin >> r >> c;
    //vector<char> temp(r, '0');
    string input;
    
    //vector<int> count(c, 0);

    // Input
    //vector<vector<char> > map(c, temp); // 2D vector
    int count[5] = {0,0,0,0,0};
    char map[5][50000];

    // map[column][row];
    for(int i = 0; i < r; i++) {
        cin >> input;
        for(int j = 0; j < c; j++) {
            map[j][i] = input[j];
        }
    }
    

    // Counts apples and obsticles in each column
    for(int i = 0; i < c; i++) {
        for(int j = 0; j < r; j++) {
            if(map[i][j] == 'a') {
                count[i]++;
                map[i][j] = '.';
            } else if(map[i][j] == '#') {
                for(int k = 1; k < count[i] + 1; k++) {
                    map[i][j-k] = 'a';
                }
                count[i] = 0;
            } 
        }
    }

    for(int i = 0; i < c; i++) {
        if(count[i] != 0) {
            for(int k = 0; k < count[i]; k++) {
                map[i][r - k - 1] = 'a';
            }
        }
    }
        
    for(int i = 0; i < r; i++) {
        for(int j = 0; j < c; j++) {
            cout << map[j][i];
        }
        cout << endl;
    }
    
    return 0;
}