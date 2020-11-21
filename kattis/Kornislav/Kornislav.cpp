include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
	int a;

	vector<int> v;

	for(int i = 0; i < 4; i++) {
		cin >> a;
		v.push_back(a);
	}

	sort(v.begin(), v.end());

	cout << v[0]*v[2] << endl;

	return 0;
}