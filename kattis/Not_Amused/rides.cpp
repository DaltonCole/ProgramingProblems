#include <iostream>
#include <map>
#include <iomanip>
using namespace std;

int main() {
	float COST = .1;

	string open;
	string word;
	string name;
	int time;
	int day = 1;
	map<string, int> enter_time;
	map<string, int> leave_time;

	while(cin >> open) {
		while(cin >> word) {
			if(word == "CLOSE") {
				break;
			} else if(word == "ENTER") {
				cin >> name;
				cin >> time;
				enter_time[name] += time;
			} else if(word == "EXIT") {
				cin >> name;
				cin >> time;
				leave_time[name] += time;
			}
		}

		cout << "Day " << day << endl;
		for(auto it : enter_time) {
			cout << it.first << " $" << setprecision(2) << fixed << COST * (leave_time[it.first] - it.second) << endl;
		}
		cout << endl;

		enter_time.clear();
		leave_time.clear();
		day++;
	}

	return 0;
}