#include <iostream>
#include <vector>
using namespace std;

int main() {

	short iterations;			// Number of iterations

	short junk;					// Junk line number
	short input;				// Line input	
	short counter;				// Counter


	vector<short> start_line;	// Starting Input
	vector<short> end_line;		// Finishing Line

	cin >> iterations;

	for(int i = 0; i < iterations; i++) {
		// Set counter to 0
		counter = 0;

		// Grab line number
		cin >> junk;

		// Gather line information
		for(int j = 0; j < 20; j++) {
			cin >> input;
			start_line.push_back(input);
		}

		// For each member of the class
		for(int j = 0; j < start_line.size(); j++) {
			// Add the initial student to the line
			if(end_line.size() == 0) {
				end_line.push_back(start_line[j]);
			} else {
				// For each child in the finished line
				for(int k = 0; k < end_line.size(); k++) {
					// If Current end line kid k is taller than current start line kid j, insert j at position k
					if(end_line[k] > start_line[j]){
						end_line.insert(end_line.begin() + k, start_line[j]);
						// Increment counter by the number of kids moved
						counter += (end_line.size() - k - 1);
						break;
					} else if(k == end_line.size() - 1) { // If we run out of kids in the end line
						// Add them to the end of the line
						end_line.push_back(start_line[j]);
						break;
					}
				}
			}
		}

		// Clear both lines
		start_line.clear();
		end_line.clear();

		// Print results
		cout << i + 1 << " " << counter << endl;
	}

	return 0;
}