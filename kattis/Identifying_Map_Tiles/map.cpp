#include <iostream>
#include <string>
#include <cmath>
using namespace std;

int main() {
	string l;
	long length, x = 0, y= 0;
	long power = 0;

	cin >> l;

	length = l.length();

	for(int i = 0; i < length; i++) {
		power = pow(2, length - i);

		if(l[i] == '0') {
			continue;
		} else if(l[i] == '1') {
			x += power / 2;
		} else if(l[i] == '2') {
			y += power / 2;
		} else if(l[i] == '3') {
			x += power / 2;
			y += power / 2;
		}
	}

	cout << length << " " << x << " " << y << endl;

	return 0;
}