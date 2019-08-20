#include <iostream>
#include <algorithm>
#include <vector>

using namespace std;

int main() {
	int n_programs;
	int m_intervals;

	vector<int> program_time;
	vector<int> interval_time;

	cin >> n_programs >> m_intervals;

	int temp;

	for(int i = 0; i < n_programs; i++) {
		cin >> temp;
		program_time.push_back(temp);
	}
	for(int i = 0; i < m_intervals; i++) {
		cin >> temp;
		interval_time.push_back(temp);
	}

	sort(program_time.begin(), program_time.end());
	sort(interval_time.begin(), interval_time.end());

	int count = 0;

	int index_p = 0, index_i = 0;

	while(index_p < program_time.size() && index_i < interval_time.size()) {
		if(program_time[index_p] <= interval_time[index_i]) {
			count++;
			index_p++;
			index_i++;
		} else {
			index_i++;
		}
	}

	cout << count << endl;

	return 0;
}