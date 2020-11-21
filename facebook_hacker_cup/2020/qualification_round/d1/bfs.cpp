// Takes WAAAAY Too Long
#include <iostream>
#include <climits>
#include <algorithm>
#include <vector>

using namespace std;

const int MAX_CITIES = 1000000;
long costs[MAX_CITIES];
int cities;
int full_gas;

const long long BIG = LLONG_MAX >> 2;

bool made_it(vector<long long> city) {
    for(const auto& n : city) {
        if(n < BIG ) {
            return true;
        }
    }
    return false;
}

long long bfs() {
    vector<long long> city(full_gas+1, BIG);
    vector<long long> next(full_gas+1, BIG);
    // Full gas in first city
    city[full_gas-1] = 0;

    for(int current_city = 1; current_city < cities-1; current_city++) {
        // Check to see if any amount of gas made it here
        if(made_it(city) == false) {
            return -1;
        }
        // Reset next
        for(auto& n : next) {
            n = BIG;
        }
        // For every gas possibility
        for(int i = 0; i <= full_gas; i++) {
            // Make sure that this state is valid
            if(city[i] >= BIG) {
                continue;
            }
            //cout << current_city << " " << i << " " << city[i] << endl;
            // Let us try keep going from this state
            if(i != 0) {
                next[i-1] = min(next[i-1], city[i]);
            }
            // Get gas 
            if(costs[current_city] != 0) {
                next[full_gas-1] = min(next[full_gas-1], city[i] + costs[current_city]);
            }       
        }
        // Swap city and next
        city.swap(next);
    }
    if(made_it(city) == false) {
        return -1;
    }
    return *min_element(city.begin(), city.end());
}

int main() {
    int cases;
    cin >> cases;
    unsigned long long answer;

    for(int i = 0; i < cases; i++) {
        cin >> cities >> full_gas;
        for(int j = 0; j < cities; j++) {
            cin >> costs[j];
        }

        answer = bfs();

        cout << "Case #" << (i + 1) << ": ";
        if(answer >= ULONG_MAX) {
            cout << -1 << endl;
        }
        else { cout << answer << endl; }
    }
    return 0;
}
