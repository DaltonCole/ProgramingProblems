#include <iostream>
#include <algorithm>
#include <utility>

using namespace std;

const int MAX_LOGS = 800000;

int position[MAX_LOGS];
int height[MAX_LOGS];
int logs;



pair<uint, uint>  dynamic_log_length(int log) {
    if(log == -1) {
        return make_pair(0, 0);
    }

}

int main() {
    int cases;
    cin >> cases;

    for(int i = 0; i < cases; i++) {
        cin >> logs;
    }
}
