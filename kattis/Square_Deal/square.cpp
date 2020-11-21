#include <iostream>
using namespace std;

int main() {
	int d[3][2]; // dimentions

	cin >> d[0][0] >> d[0][1] >> d[1][0] >> d[1][1] >> d[2][0] >> d[2][1];

	for(int i = 0; i < 3; i++) {
		for(int j = 0; j < 2; j++) {
			if(d[i][j] == d[i+1 % 3][j] + d[i+2%3][j] + d[i][j+1%2]) {
				cout << "YES" << endl;
				return 0;
			} 
			if(d[i][j] == d[i+1%3][j])
		}
	}

	for(int i = 0; i < 3; i++) {
		for(int j = 0; j < 2; j++) {
			for(int k = 0; k < 3; k++) {
				for(int l = 0; l < 2; l++) {
					if(int i != k) {
						if(d[i][j] == d[k][l] + d[k+1%3][l] + d[k+2%3][l]) {
							cout << "YES" << endl;
						}
					}
				}
			}
		}
	}




	cout << "NO" << endl;

	return 0;
}