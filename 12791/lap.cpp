#include <iostream>
using namespace std;

int main() {

	// User inputs
	float input1 = 0, input2 = 0;

	// Keep track of lap count
	int lap;

	// While there is input
	while(cin >> input1 >> input2){

		// Reset lap each test
		lap = 1;

		// Divide fast car by slow car to get speed ratio
		input1 /= input2;

		// Increment lap, until slow car is one full lap behind the fast car
		while((lap * input1) + 1 > lap) {
			lap++;
		}

		// Print Results
		cout << lap << endl;
	}

	return 0;
}