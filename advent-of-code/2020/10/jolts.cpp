#include<iostream>
#include<vector>
#include<algorithm>

using namespace std;

int main() {
    vector<int> jolts(1, 0);
    vector<int> count(4, 0);
    int tmp;

    while(cin >> tmp) {
        jolts.push_back(tmp);
    }
    sort(jolts.begin(), jolts.end());
    jolts.push_back(jolts.back() + 3);

    for(uint i = 1; i < jolts.size(); i++) {
        count[jolts[i] - jolts[i-1]]++;
    }

    cout << count[1] * count[3] << endl;

    return 0;
}
