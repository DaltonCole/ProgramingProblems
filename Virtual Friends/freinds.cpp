#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <memory>

using namespace std;

int main() {
	int cases;

	int friendships;

	string a, b;

	unordered_map<string, shared_ptr<unordered_set<string> > > friends;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> friendships;

		for(int j = 0; j < friendships; j++) {
			cin >> a >> b;

			if(friends[a] == nullptr) {
				friends[a] = shared_ptr<unordered_set<string> > (new unordered_set<string>);
			}

			friends[a] -> insert(a);
			friends[a] -> insert(b);

			if(friends[b] != nullptr && friends[a] != friends[b]) {
				friends[a] -> insert(friends[b] -> begin(), friends[b] -> end());
			}

			//delete friends[b];

			friends[b] = friends[a];

			cout << friends[a] -> size() << endl;
		}

		/*
		cout << endl;
		for(auto & map : friends) {
			cout << map.second << endl;
			if(map.second != nullptr) {
				delete map.second;
				map.second = nullptr;
			}
			cout << map.first << endl;
		}
		*/
		

		friends.clear();

	}

	return 0;
}