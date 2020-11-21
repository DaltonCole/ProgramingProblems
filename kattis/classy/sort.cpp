#include <iostream>
#include <algorithm>
#include <vector>
#include <string>

using namespace std;

int max_dashes;

char str_to_char(const string& token) {
	if(token == "upper") {
		return '1';
	} else if(token == "middle") {
		return '2';
	} else {
		return '3';
	}
}

string make_name(string& name, string& class_level, int dash_count) {
	string result = "";

	for(int i=0; i < max_dashes - dash_count; i++) {
		result += '2';
	}

	string delimiter = "-";
	size_t pos = 0;
	string token;
	while((pos = class_level.find(delimiter)) != std::string::npos) {
		token = class_level.substr(0, pos);
		result += str_to_char(token);
		class_level.erase(0, pos + delimiter.length());
	}
	result += str_to_char(class_level);

	reverse(result.begin(), result.end());

	return result + name;
}

int main() {
	int cases;
	int inputs;
	string name;
	string class_level;

	cin >> cases;

	for(int a = 0; a < cases; a++) {
		cin >> inputs;

		vector<string> names;
		vector<string> classes;
		vector<int> dash_count;

		max_dashes = 0;
		int tmp;

		// Read in data
		for(int b = 0; b < inputs; b++) {
			cin >> name;
			name.pop_back();
			cin >> class_level;
			names.push_back(name);
			classes.push_back(class_level);

			// Find number of dashes
			tmp = count(class_level.begin(), class_level.end(), '-');
			dash_count.push_back(tmp);
			if(tmp > max_dashes) {max_dashes = tmp;}

			cin >> class_level;
		}

		for(int i = 0; i < inputs; i++) {
			names[i] = make_name(names[i], classes[i], dash_count[i]);
		}

		sort(names.begin(), names.end());

		for(auto& n : names) {
			n.erase(0, max_dashes + 1);
			cout << n << endl;
		}

		cout << "==============================" << endl;
	}

	return 0;
}