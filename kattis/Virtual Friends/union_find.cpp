#include <iostream>
#include <unordered_map>

using namespace std;

#define N 100000

struct Friend {
	//int leader;
	int size;

	Friend() {
		size = 0;

	}

	Friend(int s) {
		size = s;
	}
};

int main() {
	int cases;
	int bonds;
	string name1, name2;
	unordered_map<string, Friend*> friendships;

	Friend* person1;
	Friend* person2;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> bonds;

		friendships.clear();

		for(int j = 0; j < bonds; j++) {
			cin >> name1 >> name2;

			person1 = friendships[name1];
			person2 = friendships[name2];

			// In the same set
			if(person1 == person2 && person1 != nullptr) {
				// Nothing
			}

			// Neither are in a set
			else if(person1 == person2) {
				person1 = new Friend(2);
				friendships[name1] = person1;
				friendships[name2] = person1;
			}

			// Person 2 is not in a set, but Person 1 is
			else if(person2 == nullptr) {
				friendships[name2] = person1;
				person1 -> size += 1;
			}

			// Person 1 is not in a set, but Person 2 is
			else if(person1 == nullptr) {
				friendships[name1] = person2;
				person1 = person2;
				person1 -> size += 1;
			}

			// Person 1 and Person 2 are in different sets
			else {
				for(auto dark_side : friendships) {
					if(dark_side.second == person2) {
						dark_side.second = person1;
						person1 -> size += 1;
					}
				}
			}

			cout << person1 -> size << endl;
		}

		// Delete each pointer
		for(auto dark_side : friendships) {
			dark_side.second -> size -= 1;

			if(dark_side.second -> size == 0) {
				delete dark_side.second;
			}
		}
	}

	return 0;
}