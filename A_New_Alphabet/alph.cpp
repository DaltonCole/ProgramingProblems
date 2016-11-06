#include <iostream>
#include <unordered_map>
#include <string>
using namespace std;

int main() {
	string input;
	string output;
	unordered_map<char, string> map;

	map['a'] = "@";
	map['b'] = "8";
	map['c'] = "(";
	map['d'] = "|)";
	map['e'] = "3";
	map['f'] = "#";
	map['g'] = "6";
	map['h'] = "[-]";
	map['i'] = "|";
	map['j'] = "_|";
	map['k'] = "|<";
	map['l'] = "1";
	map['m'] = "[]\\/[]";
	map['n'] = "[]\\[]";
	map['o'] = "0";
	map['p'] = "|D";
	map['q'] = "(,)";
	map['r'] = "|Z";
	map['s'] = "$";
	map['t'] = "']['";
	map['u'] = "|_|";
	map['v'] = "\\/";
	map['w'] = "\\/\\/";
	map['x'] = "}{";
	map['y'] = "`/";
	map['z'] = "2";
	map[' '] = " ";

	getline(cin, input);

	for(int i = 0; i < input.length(); i++) {
		if(map[tolower(input[i])] != "") {
			output += map[tolower(input[i])];
		} else {
			output += input[i];
		}
	}

	cout << output << endl;

	return 0;
}