#include <iostream>
#include <math.h>
using namespace std;

int main() {
	int points;
	float x1, y1, x2, y2;
	int distance;

	cin >> points;

	cin >> x1 >> y1;

	cout << "0" << endl;

	for(int i = 0; i < points; i++) {
		cin >> x2 >> y2;

		distance = round(sqrt(pow((x2-x1), 2) + pow((y2-y1), 2)));

		cout << distance << endl;;

		x1 = x2;
		y1 = y2;
	}

	return 0;
}