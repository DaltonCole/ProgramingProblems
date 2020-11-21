#include <iostream>
#include <vector>
using namespace std;

int main() {
	int cases;
	int elements;
	int temp;
	int count;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> elements;

		count = 0;

		for(int j = 0; j < elements; j++) {
			cin >> temp;
			if(temp == count + 1) {
				count++;
			}
		}

		cout << elements - count << endl;

	}
}