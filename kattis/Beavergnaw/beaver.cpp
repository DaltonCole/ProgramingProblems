#include <iostream>
#include <iomanip>
#include <cmath>
using namespace std;

int main() {
	double D;
	double V;
	double d;
	double temp;

	double pi = 3.1415926535897;

	cin >> D >> V;

	while(D != 0 && V != 0) {
		temp = (D/2) * (D/2) * pi * D;

		V = temp - V;

		d = V / (pi * D);

		cout << d << endl;

		d = 2*sqrt(d);


		cout << setprecision(15) << d << endl;

		cin >> D >> V;
	}
}