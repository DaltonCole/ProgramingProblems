#include <iostream>
#include <cmath>
#include <iomanip>

using namespace std;

int main() {
	unsigned long long int input;

	cin >> input;

	cout << setprecision(21) << (double)(4*sqrt(input)) << endl;

	return 0;
}