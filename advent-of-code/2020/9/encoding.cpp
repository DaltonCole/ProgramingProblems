#include <iostream>
#include <unordered_set>
#include <deque>

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
    T tmp;
    bool found = false;

    // Read in initial data
    for(uint i = 1; i <= MEMORY; i++) {
        cin >> tmp;
        num.push_back(tmp);
    }
    // Populate initial matrix
    for(uint i = 0; i < num.size(); i++) {
        matrix.push_back(unordered_set<T>());
        for(uint j = i + 1; j < num.size(); j++) {
           matrix[i].insert(num[i] + num[j]); 
        }
    }

    print(num, matrix);

    while(cin >> tmp) {
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
            return tmp;
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


    cout << "NOTHING FOUND?!?!?" << endl;
    return tmp;
}

int main() {
    long long num = solve<long long>();

    cout << num << endl;

    return 0;
}
