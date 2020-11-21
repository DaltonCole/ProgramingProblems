#include <iostream>
using namespace std;

int main() {
	int cases, time, task, count = 0;

	cin >> cases >> time;

	for(int i = 0; i < cases; i++) {
		cin >> task;
		time -= task;
		count++;
		if(time < 0) {
			count--;
			break;
		} else if(time == 0) {
			break;
		}
	}

	cout << count << endl;

	return 0;
}