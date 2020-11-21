// Takes WAAAAY Too Long
#include <iostream>
#include <climits>
#include <algorithm>

using namespace std;

const int MAX_CITIES = 1000000;
long costs[MAX_CITIES];
int cities;
int full_gas;

unsigned long long dynamic(int city, int gas, unsigned long long cost) {
    // If we are out of gas
    if(gas < 0) {
        return ULONG_MAX;
    }
    // If we are in the ending city
    if(city == cities - 1) {
        return cost;
    }
    if(costs[city] == 0) {
        return dynamic(city + 1, gas - 1, cost);
    }

    return min(dynamic(city + 1, full_gas-1, cost + costs[city]), dynamic(city + 1, gas - 1, cost));
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

        answer = dynamic(0, full_gas, 0);

        cout << "Case #" << (i + 1) << ": ";
        if(answer >= ULONG_MAX) {
            cout << -1 << endl;
        }
        else { cout << answer << endl; }
    }
    return 0;
}
