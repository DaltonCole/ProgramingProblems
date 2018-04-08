#include <iostream>
#include <math.h>
#include <iomanip>

using namespace std;

int main() {
	int p, a, b, c, d, n;
	float price;
	float greatest = 0;
	float difference = 0;

	cin >> p >> a >> b >> c >> d >> n;

	for(int i = 0; i < n; i++) {
		price = p * (sin((a * (i + 1)) + b) + cos((c * (i + 1)) + d) + 2); 

		if(price > greatest) {
			greatest = price;
		} else if(greatest > price && (greatest - price) > difference) {
			difference = greatest - price;
		}
	}

	cout << fixed << setprecision(6) << difference << endl;
}