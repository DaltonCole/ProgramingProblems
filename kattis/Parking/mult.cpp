#include <iostream>
using namespace std;

int main() {
	int total = 0;
	int a, b, c;
	int arrival1, leaving1;
	int arrival2, leaving2;
	int arrival3, leaving3;
	int temp;

	cin >> a >> b >> c;

	cin >> arrival1 >> leaving1;
	cin >> arrival2 >> leaving2;
	cin >> arrival3 >> leaving3;

	total += (leaving3 - arrival3) * c;

	temp = arrival3 - leaving2;

	if(temp > 0) {
		total += (leaving2 - arrival3) * c;
		leaving2 = arrival3;
	}

	temp = arrival3 - leaving1;
	
	if(temp > 0) {
		total += (leaving1 - arrival3) * c;
		leaving1 = arrival3;
	}

	total += (leaving2 - leaving1) * b;

	temp = arrival2 - leaving1;

	if(temp > 0) {
		total += (leaving1 - arrival2) * b;
	}

	total += (leaving1 - arrival1) * a;

	cout << total << endl;

	return 0;
}