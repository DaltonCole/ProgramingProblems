#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

struct reverse_sort
{
    template<class T>
    bool operator()(T const &a, T const &b) const { return a > b; }
};

int main() {
	unsigned long inputs;
	unsigned long temp;
	unsigned long total_discount = 0;

	vector<int> all_numbers;

	cin >> inputs;

	for(int i = 0; i < inputs; i++) {
		cin >> temp;
		all_numbers.push_back(temp);
	}

	sort(all_numbers.begin(), all_numbers.end(), reverse_sort());

	for(int i = 2; i < all_numbers.size(); i += 3) {
		total_discount += all_numbers[i];
	}

	cout << total_discount << endl;

	return 0;
}