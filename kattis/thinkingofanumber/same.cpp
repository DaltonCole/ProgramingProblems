include <iostream>
#include <vector>
#include <set>
#include <numeric>

using namespace std;

int main() {
	int num, constraint;
	string op, junk;
	int largest, smallest;

	while(true) {
		cin >> num;

		if(num == 0) {
			break;
		}

		set<int> divisible;
		largest = 50001;
		smallest = 0;

		for(int i = 0; i < num; i++) {
			cin >> op >> junk >> constraint;

			if(op == "less") {
				if(constraint < largest) {
					largest = constraint;
				}
			} else if(op == "greater") {
				if(constraint > smallest) {
					smallest = constraint;
				}
			} else {
				divisible.insert(constraint);
			}
		}

		if(largest == 50001) {
			cout << "infinite" << endl;
			continue;
		}

		vector<int> valid(largest - smallest - 1);
		iota(begin(valid), end(valid), smallest + 1);

		for(const auto& div : divisible) {
			for(auto& ele : valid) {
				if(ele % div != 0) {
					ele = -1;
				}
			}
		}

		vector<int> good;
		for(const auto& ele : valid) {
			if(ele != -1) {
				good.push_back(ele);
			}
		}

		if(good.size() == 0) {
			cout << "none" << endl;
		} else {
			for(const auto& i : good) {
				cout << i << " ";
			}
			cout << endl;
		}
	}

	return 0;
}