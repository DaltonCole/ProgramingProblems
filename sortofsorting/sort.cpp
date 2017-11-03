#include <iostream>
#include <set>

using namespace std;

struct Order {
	string name;
	int num;
};

struct myCompare {
	bool operator () (const Order& lhs, const Order& rhs) {
		if(lhs.name[0] != rhs.name[0]) {
			return lhs.name[0] < rhs.name[0];
		} else if(lhs.name[1] < rhs.name[1]) {
			return lhs.name[1] < rhs.name[1];
		} else {
			return lhs.num < rhs.num;
		}
	}
};

int main() {
	int count;
	string name;
	set<Order, myCompare> sort;
	Order o;

	while(true) {
		cin >> count;
		if(count == 0) {
			break;
		} else {
			sort.clear();
		}
		for(int i = 0; i < count; i++) {
			cin >> name;
			o.name = name;
			o.num = i;
			sort.insert(o);
		}

		for(auto i : sort) {
			cout << i.name << endl;
		}
		cout << endl;
	}

	return 0;
}