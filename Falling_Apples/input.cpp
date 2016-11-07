#include <iostream>
#include <random>
#include <ctime>
using namespace std;

char print() {
	int i = rand() % 10;

	if(i == -1) {
		return '#';
	} else if(i < 4) {
		return 'a';
	} else {
		return '.';
	}
}

int main() {
	srand (time(NULL));
	cout << 50000 << " " << 5 << endl;
	for(int i = 0; i < 50000; i++) {
		for(int j = 0; j < 5; j++) {
			cout << print();
		}
		cout << endl;
	}

	return 0;
}