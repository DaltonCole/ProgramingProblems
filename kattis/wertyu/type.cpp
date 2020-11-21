#include <iostream>
#include <unordered_map>

using namespace std;

int main() {
	unordered_map<char, char> m;
	m['Q'] = '\\';
	m['\\'] = ']';
	m[']'] = '[';
	m['['] = 'P';
	m['P'] = 'O';
	m['O'] = 'I';
	m['I'] = 'U';
	m['U'] = 'Y';
	m['Y'] = 'T';
	m['T'] = 'R';
	m['R'] = 'E';
	m['E'] = 'W';
	m['W'] = 'Q';

	m['A'] = '\'';
	m['\''] = ';';
	m[';'] = 'L';
	m['L'] = 'K';
	m['K'] = 'J';
	m['J'] = 'H';
	m['H'] = 'G';
	m['G'] = 'F';
	m['F'] = 'D';
	m['D'] = 'S';
	m['S'] = 'A';

	m['Z'] = '/';
	m['/'] = '.';
	m['.'] = ',';
	m[','] = 'M';
	m['M'] = 'N';
	m['N'] = 'B';
	m['B'] = 'V';
	m['V'] = 'C';
	m['C'] = 'X';
	m['X'] = 'Z';

	m['`'] = '=';
	m['='] = '-';
	m['-'] = '0';
	m['0'] = '9';
	m['9'] = '8';
	m['8'] = '7';
	m['7'] = '6';
	m['6'] = '5';
	m['5'] = '4';
	m['4'] = '3';
	m['3'] = '2';
	m['2'] = '1';
	m['1'] = '`';

	m[' '] = ' ';

	string a;
	string b;

	while(getline(cin, a)) {
		b = "";

		for(int i = 0; i < a.length(); i++) {
			b += m[a[i]];
		}

		for(int i = 0; i < b.length(); i++) {
			cout << b[i];
		}
		cout << endl;
	}

	return 0;
}