#include <iostream>
#include <vector>
#include <iomanip>
#include <math.h>

using namespace std;

int main() {
	int n;
	int s;
	double temp;
	double previous = 0;

	vector<int> n_d;
	vector<int> n_s;
	vector<int> s_d;
	vector<int> s_s;

	cin >> n >> s;

	for(int i = 0; i < n; i++) {
		cin >> temp;
		n_d.push_back(temp);
		cin >> temp;
		n_s.push_back(temp + previous);
		previous += temp;
	}

	previous = 0;
	for(int i = 0; i < s; i++) {
		cin >> temp;
		s_d.push_back(temp);
		cin >> temp;
		s_s.push_back(temp + previous);
		previous += temp;
	}

	int index_n = 0, index_s = 0;
	double total_distance = 0;



	for(int i = 0; i < n; i++) {
		total_distance += n_d[i];
	}

	double n_old_inter = 0;
	double s_old_inter = total_distance;

	double solver;

	while(true) {
		if(s_old_inter - n_old_inter < 0) {
			break;
		}

		solver = double(s_old_inter - n_old_inter) / double((double(n_d[index_n]) / double(n_s[index_n])) + (double(s_d[index_s]) / double(s_s[index_s])));

		if(isnan(solver)) {
			n_old_inter += n_d[index_n];
			index_n++;
		} else if(solver > n_s[index_n]) {
			n_old_inter += n_d[index_n];
			index_n++;
		} else if(solver > s_s[index_s]) {
			s_old_inter += s_d[index_s];
			index_s++;
		} else {
			cout << std::fixed << std::setprecision(6) << solver << endl;
			return 0;
		}
	}

	while(true) {
		solver = double(s_old_inter - n_old_inter) / double((double(n_d[index_n]) / double(n_s[index_n])) + (double(s_d[index_s]) / double(s_s[index_s])));

		if(isnan(solver)) {
			s_old_inter += s_d[index_s];
			index_s++;
		} else if(solver > n_s[index_n]) {
			n_old_inter += n_d[index_n];
			index_n++;
		} else if(solver > s_s[index_s]) {
			s_old_inter += s_d[index_s];
			index_s++;
		} else {
			cout << std::fixed << std::setprecision(6) << solver << endl;
			return 0;
		}
	}

	return 0;
}