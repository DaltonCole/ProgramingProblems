#include <iostream>
#include <algorithm>
#include <string>
using namespace std;

int main() {
	string a;
	string b;
	string x = "";
	string y = "";

	cin >> a >> b;

	reverse(a.begin(), a.end());
	reverse(b.begin(), b.end());

	int i;
	for(i = 0; i < min(a.length(), b.length()) - 1; i++) {
		if(a[i] > b[i]) {
			x += a[i];
		} else if(a[i] < b[i]) {
			y += b[i];
		} else {
			x += a[i];
			y += b[i];
		}
	}

	reverse(a.begin(), a.end());
	reverse(b.begin(), b.end());

	reverse(x.begin(), x.end());
	reverse(y.begin(), y.end());

	int diff = abs(a.length() - b.length());
	if(diff != 0) {
		int temp = 0;
		if(a.length() > b.length()) {
			while(temp < diff) {
				x = a[temp] + x;
				temp++;
			}
		} else if(a.length() < b.length()) {
			while(temp < diff) {
				y = b[temp] + y;
				temp++;
			}
		}
	}

	if(x == "") {
		cout << "YODA" << endl;
	} else {
		cout << stoi(x) << endl;
	}
	if(y == "") {
		cout << "YODA" << endl;;
	} else {
		cout << stoi(y) << endl;
	}

	return 0;
}