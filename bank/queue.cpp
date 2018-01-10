#include <iostream>
#include <queue>
#include <vector>

using namespace std;

struct Person {
	int money;
	int time;
};

class MyCompare {
	public:
		bool operator () (const Person& a, const Person& b) {
			return a.money < b.money;
		}
};

int main() {
	int people, time;

	int money, time_to_leave;

	Person person;

	priority_queue<Person, vector<Person>, MyCompare> pq;

	int total_money = 0;

	cin >> people >> time;

	vector<bool> time_slots(time, false);

	for(int i = 0; i < people; i++) {
		cin >> money >> time_to_leave;

		person.money = money;
		person.time = time_to_leave;

		pq.push(person);
	}

	while(!pq.empty()) {
		person = pq.top();
		pq.pop();

		for(int i = person.time; i >= 0; i--) {
			if(time_slots.size() >= i) {
				if(time_slots[i] == false) {
					time_slots[i] = true;
					total_money += person.money;
					break;
				}
			}
		}
	}

	cout << total_money << endl;

	return 0;
}
