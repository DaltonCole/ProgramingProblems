#include <iostream>
using namespace std;

int main() {
	int a, b;

	cin >> a >> b;

	a = b - a;
	if(a > 1) {
		cout << "Dr. Chaz will have " << a << " pieces of chicken left over!" << endl;
	} else if(a == 1) {
		cout << "Dr. Chaz will have 1 piece of chicken left over!" << endl;
	} else if(a == -1) {
		cout << "Dr. Chaz needs " << -a << " more piece of chicken!" << endl;
	} else {
		cout << "Dr. Chaz needs " << -a << " more pieces of chicken!" << endl;
	}

	return 0;
}