#include <iostream>
#include <unordered_set>
#include <deque>
#include <vector>
#include <algorithm>
#include <iterator>

using namespace std;

const uint MEMORY = 25;

// Use two deques, one for storing the original number, one for storing a set of all vales that
// number can create

template <typename T>
void print(const deque<T>& num, const deque<unordered_set<T> > matrix) {
    for(uint i = 0; i < num.size(); i++) {
        cout << num[i] << ": ";
        for(const auto& ele : matrix[i]) {
            cout << ele << " ";
        }
        cout << endl;
    }
}

template <typename T>
T solve() {
    deque<T> num;
    deque<unordered_set<T> > matrix;
    vector<T> list;
    int start = 0;
    int end = 0;
    T tmp;
    bool found = false;
    T save;

    // Read in initial data
    for(uint i = 1; i <= MEMORY; i++) {
        cin >> tmp;
        num.push_back(tmp);
        list.push_back(tmp);
    }
    // Populate initial matrix
    for(uint i = 0; i < num.size(); i++) {
        matrix.push_back(unordered_set<T>());
        for(uint j = i + 1; j < num.size(); j++) {
           matrix[i].insert(num[i] + num[j]); 
        }
    }

    while(cin >> tmp) {
        // Add to list
        list.push_back(tmp);
        // Reset found
        found = false;
        // For each num in memory
        for(const auto& row : matrix) {
            // Check if tmp exists in the row
            if(row.find(tmp) != row.end()) {
                found = true;
            }
        }
        // If no element matched, that's our bad boy!
        if(found == false) {
            save = tmp;
            break;
        }

        // Remove last element in memory
        num.pop_front();
        matrix.pop_front();
        // Add tmp to all num and all sets in matrix
        for(uint i = 0; i < num.size(); i++) {
            matrix[i].insert(num[i] + tmp);
        }
        num.push_back(tmp);
        matrix.push_back(unordered_set<T>());
    }

    // Reset count
    tmp = 0;
    // Rolling count
    for(uint i = 0; i < list.size(); i++) {
        // Update end
        end++;
        // Add new element
        tmp += list[i];

        // If new element made running total too big, shrink until it's not too big
        while(tmp > save) {
            tmp -= list[start];
            start++;
        }

        // If we found it, good job!
        if(tmp == save) {
            break;
        } 
    }

    int minimum = *min_element(next(list.begin(), start), next(list.begin(), end));
    int maximum = *max_element(next(list.begin(), start), next(list.begin(), end));


    return minimum + maximum;
}

int main() {
    long long num = solve<long long>();

    cout << num << endl;

    return 0;
}
