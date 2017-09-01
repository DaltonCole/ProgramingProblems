#include <iostream>
using namespace std;

int main() {
	int total_per_month;
	int months;
	int temp;
	int current_total = 0;

	cin >> total_per_month;
	cin >> months;

	for(int i = 0; i < months; i++) {
		cin >> temp;
		current_total += temp;
	}

	// (total possible = allowence * (months + 1)) - (total used)
	temp = (total_per_month * (months + 1)) - (current_total);

	cout << temp << endl;

	return 0;
}