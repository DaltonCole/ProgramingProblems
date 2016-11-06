#include <iostream>
#include <cmath>
#include <iomanip>
using namespace std;

int main() {
	int i;

	const double pi = 3.1415926535897;

	cin >> i;

	cout << setprecision(6) << fixed << double(i * i * pi) << endl;

	cout << setprecision(6) << fixed << double(2 * i * i) << endl;

	return 0;
}