#include <iostream>
#include <vector>

using namespace std;

int main() {
    string s;
    int i;
    int accumulator = 0;

    vector<string> op;
    vector<int> num;
    vector<bool> visited;

    while(cin >> s >> i) {
        op.push_back(s);
        num.push_back(i);
        visited.push_back(false);
    }

    i = 0;
    while(true) {
        if(visited[i] == true) {
            break;
        }
        visited[i] = true;
        if(op[i] == "nop") {
            i++;
        } else if(op[i] == "acc") {
            accumulator += num[i];
            i++;
        } else {
            i += num[i];
        }
    }

    cout << accumulator << endl;

    return 0;
}
