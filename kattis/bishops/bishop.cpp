#include <iostream>
using namespace std;

int main() {
	int n;

	while(cin >> n) {
		if(n == 1) {
			cout << 1 << endl;
		} else {
			cout << ((2 * n) - 2) << endl;
		}
	}

	return 0;
}