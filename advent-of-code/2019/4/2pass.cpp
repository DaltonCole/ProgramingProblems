#include <iostream>
#include <string>
#include <unordered_map>
using namespace std;

bool good_pass(const int num) {
    string s = to_string(num);
    // Never decrease
    for(uint i = 1; i < s.size(); i++) {
        if(s[i-1] > s[i]) {
            return false;
        }
    }
    // Adjacent same
    unordered_map<char, int> diff;
    for(const auto c : s) {
        diff[c]++;
    }
    for(const auto pair : diff) {
        if(pair.second == 2) {
            return true;
        }
    }

    return false;
}

int main() {
    int min;
    int max;
    int count = 0;

    cin >> min >> max;
    max *= -1;

    if(min < 111111) {
        min = 111111;
    }
    if(max > 999999) {
        max = 999999;
    }
    for(int i = min; i <= max; i++) {
        if(good_pass(i)) {
            count++;
        }
    }

    cout << count << endl;

    return 0;
}
