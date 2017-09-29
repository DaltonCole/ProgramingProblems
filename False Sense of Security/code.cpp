#include <iostream>
#include <unordered_map>

using namespace std;

int main(){
	unordered_map<char, char> mapping;


	mapping['?'] = '?';
	mapping['.'] = 'J';
	mapping['J'] = '.';
	mapping[','] = 'C';
	mapping['C'] = ',';
	mapping['_'] = 'Z';
	mapping['Z'] = '_';
	mapping['Y'] = 'Q';
	mapping['Q'] = 'Y';
	mapping['X'] = 'X';
	mapping['W'] = 'G';
	mapping['G'] = 'W';
	mapping['V'] = 'B';
	mapping['B'] = 'V';
	mapping['U'] = 'D';
	mapping['D'] = 'U';
	mapping['T'] = 'T';
	mapping['S'] = 'S';
	mapping['R'] = 'R';
	mapping['P'] = 'P';
	mapping['O'] = 'O';
	mapping['N'] = 'A';
	mapping['A'] = 'N';
	mapping['M'] = 'M';
	mapping['L'] = 'F';
	mapping['F'] = 'L';
	mapping['K'] = 'K';
	mapping['I'] = 'I';
	mapping['H'] = 'H';
	mapping['E'] = 'E';
	
	

}