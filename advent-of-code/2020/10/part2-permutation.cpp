#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;

int main() {
    vector<int> jolts(1, 0);
    vector<int> count;
    int tmp;

    while(cin >> tmp) {
        jolts.push_back(tmp);
    }
    sort(jolts.begin(), jolts.end());
    jolts.push_back(jolts.back() + 3);

    for(uint i = 0; i < jolts.size() - 1; i++) {
        tmp = i+1;
        count.push_back(0);
        while(jolts[i] + 3 >= jolts[tmp] && (uint)tmp < jolts.size() && count[count.size() - 1] < 2) {
            count[count.size() - 1]++;
            tmp++;
        }
    }

    for(uint i = 0; i < count.size(); i++) {
        cout << jolts[i] << ": " << count[i] << endl;
    }

    unsigned long long total = 1;
    int two = 0;
    int seven = 0;
    int twos_in_a_row = 0;

    // Count the number of 2s. When three 2s appear in a row, there are only 7 different possibilities,
    //  not 8.
    //  For Example, look at 1, 2, 3, and 4:
    //  0,1,2,3,4
    //  0,1,2,4
    //  0,1,3,4
    //  0,1,4
    //  0,2,3,4
    //  0,2,4
    //  0,3,4
    //  => 7 possibilities
    for(const int i : count) {
        if(i == 2) {
            two++;
            twos_in_a_row++;
            if(twos_in_a_row == 3) {
                twos_in_a_row = 0;
                two -= 3;
                seven++;
            }
        }
        if(i == 1) {
            twos_in_a_row = 0;
        }
    }

    for(int i = 0; i < two; i++) {
        total *=2;
    }
    for(int i = 0; i < seven; i++) {
        total *= 7;
    }

    cout << total << endl;

    return 0;
}
