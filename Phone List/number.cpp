#include <iostream>
#include <vector>
#include <ctime>

using namespace std;

int main() {
	std::clock_t start = std::clock();


	int test_cases;
	int inputs;
	string number;

	vector<string> many_numbers;

	bool exit = false;

	cin >> test_cases;

	for(int i = 0; i < test_cases; i++) {
		cin >> inputs;

		for(int j = 0; j < inputs; j++) {
			cin >> number;

			for(int k = 0; k < many_numbers.size(); k++) {

				if(number.length() < many_numbers[k].length()) {
					continue;
				}

				for(int l = 0; l < many_numbers[k].length(); l++){
					if(number[l] == many_numbers[k][l]) {
						exit = true;
					}
					else{
						exit = false;
						break;
					}
				}

				if(exit == true){
					break;
				}
			}

			if(exit == true){
				break;
			}

			many_numbers.push_back(number);
		}

		if(exit == true) {
			cout << "NO" << endl;
		}
		else {
			cout << "YES" << endl;
		}

		exit = false;
		many_numbers.clear();
	}

	cout << ( std::clock() - start ) / (double) CLOCKS_PER_SEC << endl;

	return 0;
}