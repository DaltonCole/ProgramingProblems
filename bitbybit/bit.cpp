#include <iostream>
using namespace std;

void change(string & bits, string & command, int & b1, int & b2) {
	cin >> command;

	if(command == "CLEAR") {
		cin >> b1;
		bits[31 - b1] = '0';
	} else if(command == "SET") {
		cin >> b1;
		bits[31 - b1] = '1';
	} else if(command == "AND") {
		cin >> b1 >> b2;
		if(bits[31-b1] == '0' || bits[31-b2] == '0') {
			bits[31-b1] = '0';
		} else if(bits[31-b1] == '?' || bits[31-b2] == '?') {
			bits[31-b1] = '?';
		} else {
			bits[31-b1] = '1';
		}
	} else if(command == "OR") {
		cin >> b1 >> b2;
		if(bits[31-b1] == '1' || bits[31-b2] == '1') {
			bits[31-b1] = '1';
		} else if(bits[31-b1] == '?' || bits[31-b2] == '?') {
			bits[31-b1] = '?';
		} else {
			bits[31-b1] = '0';
		}
	}

	return;
}

int main() {
	string bits;
	int cases;

	string command;
	int b1, b2;

	while(true) {
		cin >> cases;
		if(cases == 0) {
			break;
		}
		bits = "????????????????????????????????";

		for(int i = 0; i < cases; i++) {
			change(bits, command, b1, b2);
		}

		cout << bits << endl;
	}

	return 0;
}