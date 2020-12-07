#include <iostream>
#include <vector>

using namespace std;

int main() {
    vector<string> map;
    string tmp;
    int count = 0;

    // Read in map
    while(cin >> tmp) {
        map.push_back(tmp);
    }

    int col = 0;
    for(const auto& row : map) {
        if(row[col] == '#') {
            count++;
        }
        col = (col + 3) % row.length();
    }

    cout << count << endl;

    return 0;
}
