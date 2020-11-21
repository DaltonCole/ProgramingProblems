#include <iostream>
#include <iomanip>
using namespace std;

int main() {
	double r, m, c;
	double area, approx;

	double pi = 3.1415926535897;

	cin >> r >> m >> c;

	while(r != 0 && m != 0 && c != 0) {
		area = r * r * pi;
		approx = m / c;
		approx = ((r + r) * (r + r)) / approx;


		cout << setprecision(15) << area << " " << setprecision(15) << approx << endl;

		cin >> r >> m >> c;
	}

	return 0;
}