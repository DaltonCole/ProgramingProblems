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
    // Keep track of all possible jmp and nop instructions
    vector<int> alter;

    while(cin >> s >> i) {
        op.push_back(s);
        num.push_back(i);
        visited.push_back(false);
        if(s == "nop" || s == "jmp") {
            alter.push_back(op.size() - 1);
        }
    }

    for(const auto alt : alter) {
        i = 0;
        accumulator = 0;
        // Alter instruction
        if(op[alt] == "nop") {
            op[alt] = "jmp";
        } else {
            op[alt] = "nop";
        }
        while(true) {

            if(i == op.size()) {
                cout << alt << endl;
                cout << accumulator << endl;
                return 0;
            }

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

        // Reset visited
        for(int x = 0; x < visited.size(); x++) {
            visited[x] = false;
        }

        // Alter instruction back
        if(op[alt] == "nop") {
            op[alt] = "jmp";
        } else {
            op[alt] = "nop";
        }
    }

    return 0;
}
