#include <iostream>
#include <unordered_set>
#include <unordered_map>
#include <list>
#include <stack>

using namespace std;

string goal;

string delimit_by_space(string & s, bool & end);

struct Move {
	string current_location;
	unordered_set<string> past_locations;
	list<string> order;

	int add_destination(const string & dest) {
		if(dest == goal) {
			current_location = dest;
			order.push_back(dest);
			return 2;
		}

		auto got = past_locations.find(dest);

		if(got != past_locations.end()) {
			return 0;
		}

		past_locations.insert(dest);
		order.push_back(dest);
		current_location = dest;

		return 1;
	}
};



int main() {
	int places;
	string source, destination;
	string other_place;
	bool end = false;

	unordered_map<string, list<string> > travel_to;

	cin >> places;

	for(int i = 0; i < places; i++) {
		cin >> source;
		getline(cin, destination);
		end = false;
		while(!end) {
			end = true;
			other_place = delimit_by_space(destination, end);
			travel_to[source].push_back(other_place);
		}

		
	}

	cin >> source >> destination;
	goal = destination;

	stack<Move> dfs;
	Move m;
	Move temp_m;
	m.add_destination(source);
	dfs.push(m);
	bool done = false;
	int test;
	while(!dfs.empty() && !done) {
		m = dfs.top();
		dfs.pop();

		for(auto i : travel_to[m.current_location]) {
			temp_m = m;
			test = temp_m.add_destination(i);
			if(test == 2) {
				for(auto k : temp_m.past_locations) {
					cout << k << " ";
				}
				cout << destination << " ";
				done = true;
				break;
			} else if(test == 1) {
				dfs.push(temp_m);
				for(auto k : temp_m.past_locations) {
					cout << k << " ";
				}
				cout << endl;
			}
		}
	}

	if(!done) {
		cout << "no route found";
	}
	cout << endl;

	
	for(auto i : travel_to) {
		cout << i.first << ": ";
		for(auto j : i.second) {
			cout << j << " ";
		}
		cout << endl;
	}
	

	return 0;
}

string delimit_by_space(string & s, bool & end) {
	string a = "";
	int i = 0;

	for(i = 1; i < s.length(); i++) {
		if(s[i] == ' '){
			end = false;
			break;
		} else if(s[i] != '\n') {
			a += s[i];
		}
	}

	string temp = "";
	for(int j = i; j < s.length(); j++) {
		temp += s[j];
	}
	s = temp;

	return a;
}