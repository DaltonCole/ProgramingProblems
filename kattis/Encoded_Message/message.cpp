#include <iostream>
#include <cmath>
#include <algorithm>
#include <string>
#include <vector>

using namespace std;

int main() {
	int cases = 0;
	string message;
	string answer;
	vector<char> temp;
	vector<vector<char> > table;
	int part;
	int size;

	cin >> cases;

	for(int q = 0; q < cases; q++) {
		cin >> message;

		size = ceil(sqrt(message.length()));
		part = 0;

		for(int i = 0; i < size; i++) {
			temp.push_back('*');
		}
		for(int i = 0; i < size; i++) {
			table.push_back(temp);
		}


		for(int i = 0; i < size; i++) {
			for(int j = 0; j < size; j++) {
				if(part < message.length()) {
					table[i][j] = message[part];
					part++;
				} else {
					break;
				}
			}
		}

		for(int i = size - 1; i >= 0; i--) {
			for(int j = 0; j < size; j++) {
					answer += table[j][i];
			}
		}

		cout << answer << endl;

		answer = "";
		temp.clear();
		table.clear();

	}
}