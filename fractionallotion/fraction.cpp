#include <iostream>
using namespace std;


bool fraction(int y, int z) {
	int den = y - z;

	if(den <= 0) {
		return false;
	}

	int num = y * z;

	if(num % den == 0) {
		return true;
	}

	return false;
}

int main() {
	string line;
	
	int z;


	while(cin >> line) {
		string s_den = "";
		for(int i = 2; i < line.size(); i++) {
			s_den += line[i];
		}

		int count = 0;

		z = stoi(s_den);


		// x fraction cant be 0, so y = z + 1
		// x and y must be greater than z
		// After z ^ 2, x and y are flipped
		for(int y = z + 1; y < (z << 1) + 1; y++) {
			if(fraction(y, z)) {
				count++;
			}
		}

		cout << count << endl;
	}

	return 0;
}