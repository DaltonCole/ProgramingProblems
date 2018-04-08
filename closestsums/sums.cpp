#include <iostream>
#include <algorithm>
#include <climits>
#include <cmath>
using namespace std;

int main() {
	int case_number = 1;
	int elements;
	int queries;
	int temp;
	long difference;
	int start; // 
	int end;

	int result_small;
	int result_large;

	int* array;

	while(cin >> elements) {
		array = new int[elements];

		for(int i = 0; i < elements; i++) {
			cin >> temp;
			array[i] = temp;
		}

		// Sort
		sort(array, array + elements);

		cin >> queries;

		cout << "Case " << case_number << ":" << endl;
		for(int i = 0; i < queries; i++) {
			cin >> temp;
			difference = LONG_MAX;
			start = 0;
			end = elements - 1;
			result_small = start;
			result_large = end;
			while(start < end) {
				if(abs(array[start] + array[end] - temp) < difference) {
					difference = abs(array[start] + array[end] - temp);
					result_small = start;
					result_large = end;
				} else if(array[start] + array[end] < temp) {
					start++;
				} else {
					end--;
				}
			}

			cout << "Closest sum to " << temp << " is " << (array[result_small] + array[result_large]) << "." << endl;

		}
		case_number++;


		delete[] array;
	}

	return 0;
}