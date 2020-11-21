#include <iostream>
#include <climits>
#include <algorithm>
#include <vector>
#include <map>

using namespace std;

const int MAX_CITIES = 1000000;
long costs[MAX_CITIES];
int cities;
int full_gas;

const long long BIG = LLONG_MAX >> 2;

void prune(map<uint, long long>& city) {
    // Tag any gas amount where more gas has less than or equal cost
    long long min_cost = BIG;
    for(auto it = city.rbegin(); it != city.rend(); it++) {
        // If cost is greater than current smallest cost, remove
        if(it -> second >= min_cost) {
            it -> second = BIG;
        } else {
            min_cost = it -> second;
        }
    }
    // Remove any cost that is greater than BIG
    for(auto it = city.cbegin(); it != city.cend();) {
        if(it -> second >= BIG) {
            city.erase(it++);
        } else {
            ++it;
        }
    }
    
}

bool made_it(map<uint, long long>& city) {
    // Prune - Remove any cost that is greater than BIG
    prune(city); 
    if(city.size() > 0) {
        return true;
    }
    return false;
}

long long value(const uint element, const map<uint, long long> next) {
    auto it = next.find(element);
    if(it != next.end()) {
        return it -> second;
    }
    return BIG;
}

long long bfs() {
    map<uint, long long> city;
    map<uint, long long> next;
    // Full gas in first city
    city[full_gas-1] = 0;

    for(int current_city = 1; current_city < cities-1; current_city++) {
        // Check to see if any amount of gas made it here
        if(made_it(city) == false) {
            return -1;
        }
        // Reset next
        next.clear();
        // For every gas possibility
        for(auto it = city.cbegin(); it != city.cend(); it++) {
            //cout << current_city << " " << (it -> second) << " " << (it -> first) << endl;
            // Keep going, who needs gas anyways?
            if(it -> first != 0) {
                next[it -> first - 1] = min(it -> second, value(it -> first - 1, next));
            }
            // Get gas
            if(costs[current_city] != 0) {
                next[full_gas-1] = min(it -> second + costs[current_city], value(full_gas-1, next));
            }
        }
        // Swap city and next
        city.swap(next);
    }
    if(made_it(city) == false) {
        return -1;
    }
    return city.begin() -> second;
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
