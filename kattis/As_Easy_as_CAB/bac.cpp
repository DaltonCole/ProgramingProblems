#include <iostream>
#include <vector>
#include <algorithm>
#include <string>

using namespace std;


struct compare {
	bool operator() (string a, string b) {
		return a[a.length() - 1] == b[0];
	}
} comp;


int main() {
	char letter;	// highest letter
	int cases;		// Number of strings
	
	int WHAT = 97; // 'a'
	
	cin >> letter >> cases;	

	vector<char> single_set;
	vector<vector<char> > sets;		// Vector of char char pairs

	// For input
	vector<string> input;
	string temp;

	vector<string> letter_vector;

	// Read in strings
	for(int i = 0; i < cases; i++) {
		cin >> temp;
		input.push_back(temp);
	}

	bool leave = false;
	// Read in strings
	for(int i = 0; i < cases; i++) {
		temp = input[i];
		cin >> temp;
		for(int j = 0; j < temp.size(); j++) {
			if(j > 0 && i > 0){
				for(int k = 1; k <= j; k++) {
					if(input[i-1][j-k] != input[i][j-k]) {
						leave = true;
						break;
					}
				}
				if(leave){
					leave = false;
					break;
				}
			}
			if(letter_vector.size() < j + 1) {
				letter_vector.push_back("");
			}
			letter_vector[j] += temp[j];
		}
	}

	for(int i = letter_vector.size() - 1; i > 0; i--) {
		if(letter_vector[i].size() == 1) {
			letter_vector.erase(letter_vector.begin()+i);
		}
	}

	for(auto & it : letter_vector) {
		auto resize_it = unique(it.begin(), it.end());
		it.resize(distance(it.begin(), resize_it));
	}

	sort(letter_vector.begin(), letter_vector.end(), comp);

	bool amb = false;
	for(int i = letter_vector.size() - 1; i > 0; i--) {
		if(letter_vector[i][0] == letter_vector[i-1][letter_vector[i-1].size() - 1]) {
			letter_vector[i-1].pop_back();
			letter_vector[i-1] += letter_vector[i];
			letter_vector.erase(letter_vector.end() - 1);
		} else {
			amb = true;
		}
	}

	temp = letter_vector[0];

	bool forb = false;
	int temp_size = temp.size();
	for(int i = 0; i < temp_size; i++) {
		for(int j = i + 1; j < temp_size; j++) {
			if(temp[i] == temp[j]) {
				forb = true;
			}
		}
	}

	auto resize_temp = unique(temp.begin(), temp.end());
	temp.resize(distance(temp.begin(), resize_temp));
	//cout << temp << endl;

	if(forb) {
		cout << "IMPOSSIBLE" << endl;
	} else if(amb || letter_vector[0].size() != letter - WHAT + 1) {
		cout << "AMBIGUOUS" << endl;
	}  else {
		cout << letter_vector[0] << endl;
	}

/*
	for(int i = 0; i < letter_vector.size(); i++) {
		for(int j = 0; j < letter_vector[i].size(); j++) {
			cout << letter_vector[i][j];
		}
		cout << endl;
	}
*/
	


	/*
	// Create sets
	for(int i = 0; i < input.size() - 1; i++) {
		for(int j = 0; j < min(input[i].length(), input[i+1].length()); j++) {
			if(input[i][j] != input[i+1][j]){
				if(j > 0) {
					if(input[i][j-1] != input[i+1][j-1]) {
						break;
					}
				}
				cout << input[i][j] << endl;
				if(!insert_element(single_set, input[i][j])) {
					cout << input[i][j] << " " << input[i+1][j] << endl;
					cout << "IMPOSSIBLE" << endl;
					return 0;
				}
			}
		}
		sets.push_back(single_set);
		single_set.clear();
	}

	for(auto it : sets) {
		for(auto it2 : it) {
			cout << it2 << " ";
		}
		cout << endl;
	}
	*/
	
	return 0;
}
