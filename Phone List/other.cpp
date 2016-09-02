#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

struct compare {
    bool operator()(const std::string& first, const std::string& second) {
        return first.size() < second.size();
    }
};

int main() {
	std::clock_t start = std::clock();
	int test_cases;
	int inputs;
	string number;

	compare c;

	vector<string> many_numbers;

	bool exit = false;

	cin >> test_cases;

	for(int i = 0; i < test_cases; i++) {
		cin >> inputs;

		for(int j = 0; j < inputs; j++) {
			cin >> number;

			many_numbers.push_back(number);
		}

		sort(many_numbers.begin(), many_numbers.end(), c);

		for(int j = 1; j < many_numbers.size(); j++) {
			for(int k = 0; k < j; k++) {
				for(int l = 0; l < many_numbers[k].size(); l++) {
					if(many_numbers[k][l] == many_numbers[j][l]) {
						exit = true;
					}
					else {
						exit = false;
						break;
					}
				}
				if(exit == true) {
					break;
				}
			}
		}

		if(exit == true) {
			cout << "NO" << endl;
		}
		else{
			cout << "YES" << endl;
		}

		exit = false;
		many_numbers.clear();
	}
	cout << ( std::clock() - start ) / (double) CLOCKS_PER_SEC << endl;

	return 0;
}