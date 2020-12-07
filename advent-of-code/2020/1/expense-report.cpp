#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

const int TOTAL = 2020;

int main() {
    vector<int> num;    // Holds the expense report
    int tmp;
    int start;  // Move up the expense report
    int end;    // Move down the expense report

    // Read in data
    while(cin >> tmp) {
        num.push_back(tmp);
    }

    // Sort expense report from smallest to largest number
    sort(num.begin(), num.end());

    // Set start and end
    start = 0;
    end = num.size() - 1;

    // Check all pairs of numbers (order does not matter)
    while(start < end) {
        // On the mark!
        if(num[start] + num[end] == TOTAL) {
            cout << (num[start] * num[end]) << endl;
            break;
        } 
        // Too big, make larger number smaller
        else if(num[start] + num[end] > TOTAL) {
            end--;
        }
        // Too small, make smaller number bigger
        else {
            start++;
        }
    }

    return 0;
}
