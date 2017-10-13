#include <iostream>
#include <unordered_set>
#include <algorithm>

using namespace std;

int main() {
	// How many test cases and how much words to read in
	int cases, numbers;

	// Number to read in and every sub number of it
	string number, sub_number;

	// Set of all full number strings
	unordered_set<string> all_numbers;

	// Iterator for find
	unordered_set<string>::const_iterator got;

	// Vector of all inputs
	vector<string> phone_numbers;

	// See if sub_string is in set
	bool found;

	// Read in the number of cases
	cin >> cases;

	for(int i = 0; i < cases; i++) {
		// Read in the number of inputs
		cin >> numbers;

		// Initalize found to false
		found = false;

		// Clear phone numbers
		phone_numbers.clear();
		all_numbers.clear();

		// For each phone number
		for(int j = 0; j < numbers; j++) {
			// Read in phone number
			cin >> number;

			// Add to vector
			phone_numbers.push_back(number);
		}

		// Sort vector
		sort(phone_numbers.begin(), phone_numbers.end());

		// For each read in item
		for(auto num : phone_numbers){
			// If we already found a match read in rest of inputs
			if(found == true) {
				break;	
			}

			// Initalize sub_number as an empty string
			sub_number = "";

			// For each letter in string
			for(int w = 0; w < num.length(); w++) {
				// Add character to substring
				sub_number += num[w];

				// Look for number in substring
				got = all_numbers.find(sub_number);

				// If we found it, set found to true
				if(got != all_numbers.end()) {
					found = true;
					//cout << sub_number << endl;
					break;
				}
			}

			// Add phone number to number list
			all_numbers.insert(num);
		}

		if(found == true) {
			cout << "NO" << endl;
		} else {
			cout << "YES" << endl;
		}
	}
}