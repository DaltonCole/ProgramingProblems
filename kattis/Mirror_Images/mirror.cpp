#include <iostream>
#include <vector>
using namespace std;

int main() {
	int cases = 0;
	int r = 0;
	int c = 0;
	string input;
	vector<char> temp;
	vector<vector<char> > mirror;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> r >> c;

		for(int j = 0; j < r; j++) {
			temp.push_back(' ');
		}
		for(int j = 0; j < c; j++) {
			mirror.push_back(temp);
		}
		//cout << r << " " << c << endl;
		//cout << mirror.size() << endl;
		//cout << "hi" << endl;
		for(int j = 0; j < r; j++) {
			cin >> input;
			//cout << input << endl;
			for(int k = 0; k < c; k++) {
				mirror[j][k] = input[k];
			}
		}

		cout << "Test " << i+1 << endl;
		//cout << mirror.size() << endl;
		//cout << "z" << endl;

		for(int i = mirror.size() - 1; i >= 0; i--) {
			for(int j = mirror[i].size() - 1; j >= 0; j--) {
				//cout << i << " " << j << endl;
				cout << mirror[i][j];
			}
			cout << endl;
		}

		temp.clear();
		mirror.clear();
	}

	return 0;
}