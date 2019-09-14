#include <iostream>
#include <vector>
#include <set>

using namespace std;

set<int> primes = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71};

void get_primes() {
	for(int i = 73; i < 32000; i++) {
		bool good = true;

		for(const auto& num : primes) {
			if(i % num == 0) {
				good = false;
				break;
			}
		}

		if(good) {
			primes.insert(i);
		}
	}
}

int main() {
	get_primes();

	int cases;
	cin >> cases;

	int number;

	for(int i = 0; i < cases; i++) {
		vector<int> result; 

		cin >> number;

		for(const auto& num : primes) {
			if((num * 2) > number) {break;}

			if(primes.find(number - num) != primes.end()) {
				result.push_back(num);
			}
		}

		cout << number << " has " << result.size() << " representation(s)" << endl;
		for(const auto& num : result) {
			cout << num << "+" << (number - num) << endl;
		}
		cout << endl;
	}
}

