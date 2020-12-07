#include <iostream>
using namespace std;

int main() {
    int total = 0;
    int input;

    while(cin >> input) {
        while(input > 0) {
            input /= 3;
            input -= 2;
            if(input > 0) {
                total += input;    
            }
        }
    }

    cout << total << endl;

    return 0 ;
}
