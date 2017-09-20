#include <iostream>
#include <unordered_map>

using namespace std;

void split_string(string & input1, string & input2) {
	string in1 = "";
	input2 = "";

	int i;
	for(i = 0; i < input1.length(); i++) {
		if(input1[i] == ' ') {
			break;
		}
		in1 += input1[i];
	}
	i++;
	for(;i < input1.length(); i++) {
		input2 += input1[i];
	}

	input1 = in1;
}

int main() {
	unordered_map<string, string> my_dict;
	string input1, input2;

	while(getline(cin, input1)) {
		if(input1.empty()) {
			break;
		}
		split_string(input1, input2);

		my_dict[input2] = input1;
	}

	while(cin >> input1) {
		try {
			cout << my_dict.at(input1) << endl;
		}
		catch( ...) {
			cout << "eh" << endl;
		}
	}

	return 0;
}