#include <iostream>
#include <vector>
#include <utility>

using namespace std;

int trees(const int right, const int down, const vector<string> map) {
    int count = 0;
    int col = 0;
    for(uint i = 0; i < map.size(); i += down) {
        if(map[i][col] == '#') {
            count++;
        }
        col = (col + right) % map[i].length();
    }
    return count;
}

int main() {
    vector<string> map;
    vector<pair<int, int> > combination = {{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}};
    string tmp;
    long count = 1;

    // Read in map
    while(cin >> tmp) {
        map.push_back(tmp);
    }

    for(const auto comb : combination) {
        count *= trees(comb.first, comb.second, map);
    }

    cout << count << endl;

    return 0;
}
