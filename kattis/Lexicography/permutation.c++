include <iostream>
#include <algorithm>
using namespace std;
 
int main()
{
    string s;
    unsigned long long num;

    while(cin >> s >> num) {
    	if(s == "#") {
    		break;
    	}
    	sort(s.begin(), s.end());

    	while(num != 1) {
    		num--;
    		next_permutation(s.begin(), s.end());
    	}

    	cout << s << endl;
    }

    return 0;
}