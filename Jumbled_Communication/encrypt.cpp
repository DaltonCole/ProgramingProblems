#include <iostream>
using namespace std;

int main() {
	int output;

	for(int i = 0; i < 256; i++) {
		output = (i ^ (i << 1)) % 256;
		cout << "map[" << output << "] = " << i << ";" << endl;
	}

	return 0;
}