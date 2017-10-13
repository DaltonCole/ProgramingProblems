#include <iostream>
#include <map>

using namespace std;

struct classcomp {
  bool operator() (const int& lhs, const int& rhs) const
  {return lhs > rhs;}
};

int main() {
	map<string, int, classcomp> m;

	string text;
	int top;

	while(cin >> text) {
		if(text == "<text>" or text == "</text>" or text == "/>") {
			continue;
		} else if(text == "<top") {
			cin >> top;
		}
	}



}