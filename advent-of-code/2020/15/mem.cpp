#include <iostream>
#include <unordered_map>

using namespace std;

const int FINISH = 30000000;

int main() {
    int count = 0;
    int num;
    int old = -1;
    unordered_map<int, int> mem;

    while(cin >> num) {
        count++;
        mem[num] = count;
    }

    while(count < FINISH) {
        count++;

        if(old == -1) {
            num = 0;
        } else {
            num = count - old - 1;
        }

        if(mem.find(num) != mem.end()) {
            old = mem[num];
        } else {
            old = -1;
        }

        mem[num] = count;
    }

    cout << num << endl;

    return 0;
}
