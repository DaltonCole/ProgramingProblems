#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;


struct compare {
	bool operator() (pair<char, char> a, pair<char, char> b) {
		return a.second == b.first;
	}
} comp;

struct unique_set {
	bool operator () (pair<char, char> a, pair<char, char> b) {
		if(a.first == b.first && a.second == b.second) {
			return true;
		} else {
			return false;
		}
	}
} u_set;

bool impossible(pair<char, char> a, pair<char, char> b) {
	if(a.first == b.second && b.first == a.second) {
		return false;
	} else {
		return true;
	}
}


int main() {
	char letter;	// highest letter
	int cases;		// Number of strings
	
	int WHAT = 97; // 'a'
	
	cin >> letter >> cases ;	

	vector<pair<char, char> > set;	// Vector of char char pairs
	
	vector<string> input;			// Input strings
	
	string temp;
	
	auto p = make_pair('a', 'a');
	
	vector<char> t;	// Vector containing all of the letters, for ambigous
	
	// Read in strings
	for(int i = 0; i < cases; i++) {
		cin >> temp;
		input.push_back(temp);
	}
	
	// Create sets
	for(int i = 0; i < input.size() - 1; i++) {
		for(int j = 0; j < min(input[i].length(), input[i+1].length()); j++) {
			if(input[i][j] != input[i+1][j]){
				p.first = input[i][j];
				p.second = input[i+1][j];
				set.push_back(p);
				break;
			}
		}
	}

	// Only unique sets
	unique(set.begin(), set.end(), u_set);
	
	// Sort the sets such that a.second = b.first
	sort(set.begin(), set.end(), comp);
/*
	for(int i = set.size() - 1; i > 0; i--) {
		if(set[i].first != set[i-1].second) {
			set.erase(set.begin() + (i));
		}
	}

	if(set.size() > 2) {
		if(set[0].second != set[1].first) {
			set.erase(set.begin());
		}
	}
//*/
	for(auto it : set) {
		cout << it.first << " " << it.second << endl;
	}
	
	// Add all of the letters in the sets
	for(auto i : set) {
		t.push_back(i.first);
	}
	t.push_back(set[set.size() - 1].second);


	vector<char> u = t;
	// Take only unique ones
	sort(t.begin(), t.end());
	auto m = unique(t.begin(), t.end()); 
	t.resize( std::distance(t.begin(),m) );
	
	// If we don't have enough letters, then it is AMBIGUOUS
	if(letter - WHAT - 1 > t.size()) {
		cout << "AMBIGUOUS" << endl;
		return 0;
	} 

	if(u.size() == t.size()) {
		for(auto it : u) {
			cout << it;
		}
		cout << endl;
	} else {
		cout << "IMPOSSIBLE" << endl;
	}
	
	/*
	bool imp = true;
	for(int i = 0; i < set.size() - 1; i++) {
		imp = impossible(set[i], set[i+1]);
		if(imp == false){
			break;
		}
	}
	
	if(set[0].first == set[set.size() - 1].second || !imp) {
		cout << "IMPOSSIBLE" << endl;
	} else{
		for(auto i : t) {
			cout << i;
		}
		cout << endl;
	}*/
	
	
	return 0;
}
