#include <iostream>
#include <cmath>

using namespace std;

#define N 1000

int arr[N];
int prefix_sum[N];

int main() {
	int cases;
	int temp;
	int actual_number;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> temp;
		arr[i] = temp;
	}

	// First element in prefix sum
	prefix_sum[0] = arr[0];

	for(int i = 1; i < cases; i++) {
		prefix_sum[i] = prefix_sum[i-1] + arr[i];
	}

	// Find difference closest to 1000
	temp = abs(1000 - prefix_sum[0]);
	actual_number = prefix_sum[0];

	// For each element
	for(int i = 0; i < cases; i++) {
		// If index i == 1000, we are done
		if(prefix_sum[i] == 1000) {
			cout << prefix_sum[i] << endl;
			return 0;
		}

		// For each element greater than the ith element
		for(int j = i + 1; j < cases; j++) {
			// If the difference between the jth element and ith is less than temp
			if(abs(prefix_sum[j] - prefix_sum[i] - 1000) <= temp) {
				// Check to see if temp is equal to current difference
				if(temp == abs(prefix_sum[j] - prefix_sum[i] - 1000)) {
					// If so, only values greater than 1000
					if((prefix_sum[j] - prefix_sum[i]) >= 1000) {
						actual_number = prefix_sum[j] - prefix_sum[i];
					}
				} else {
					// If new temp is less than old temp, go ahead and set actual number
					actual_number = prefix_sum[j] - prefix_sum[i];
				}
				// Set temp
				temp = abs(prefix_sum[j] - prefix_sum[i] - 1000);
			}
		}
	}

	// Consider all of the elements
	if(abs(prefix_sum[cases - 1] - 1000) <= temp) {
		actual_number = prefix_sum[cases-1];
	}

	cout << actual_number << endl;

	return 0;
}