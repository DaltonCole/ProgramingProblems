include <iostream>
#include <unordered_set>
#include <queue>
#include <utility>

using namespace std;

struct Safe {
	int num[3][3];

	void print() {
		for(int i = 0; i < 3; i++) {
			for(int j = 0; j < 3; j++) {
				cout << num[i][j];
			}
			cout << endl;
		}
	}

	bool operator==(const Safe& rhs) {
		for(int i = 0; i < 3; i++) {
			for(int j = 0; j < 3; j++) {
				if(num[i][j] != rhs.num[i][j]) {
					return false;
				}
			}
		}
		return true;
	}

	bool solved() {
		for(int i = 0; i < 3; i++) {
			for(int j = 0; j < 3; j++) {
				if(num[i][j] != 0) {
					return false;
				}
			}
		}
		return true;
	}
};

bool operator==(const Safe& lhs, const Safe& rhs) {
	for(int i = 0; i < 3; i++) {
		for(int j = 0; j < 3; j++) {
			if(lhs.num[i][j] != rhs.num[i][j]) {
				return false;
			}
		}
	}
	return true;
}

namespace std {
	template<>
	struct hash<Safe> {
		size_t operator()(const Safe& s) const {
			return 	hash<int>()(s.num[0][0]) ^
					hash<int>()(s.num[0][1] + 10) ^ 
					hash<int>()(s.num[0][2] + 20) ^ 
					hash<int>()(s.num[1][0] + 30) ^ 
					hash<int>()(s.num[1][1] + 40) ^ 
					hash<int>()(s.num[1][2] + 50) ^ 
					hash<int>()(s.num[2][0] + 60) ^ 
					hash<int>()(s.num[2][1] + 70) ^ 
					hash<int>()(s.num[2][2] + 80);
		}
	};
}

class myComparison {
	public:
		bool operator() (const pair<Safe, int>& lhs, const pair<Safe, int>& rhs) {
			bool tmp = lhs.second > rhs.second;
			return tmp;
		}
};


int main() {
	Safe toy;
	pair<Safe, int> current_safe;

	priority_queue<pair<Safe, int>, vector<pair<Safe, int> >, myComparison> pq;
	unordered_set<Safe> seen_safes;

	for(int i = 0; i < 3; i++) {
		for(int j = 0; j < 3; j++) {
			cin >> toy.num[i][j];
		}
	}

	pq.push(make_pair(toy, 0));

	while(!pq.empty()) {
		current_safe = pq.top();
		pq.pop();

		if(current_safe.second > 1000) {
			break;
		}

		if(current_safe.first.solved() == true) {
			break;
		}

		for(int i = 0; i < 3; i++) {
			for(int j = 0; j < 3; j++) {
				toy = current_safe.first;

				// i = 0, j = 1
				for(int k = 0; k < 3; k++) {
					toy.num[i][k]++;
					toy.num[k][j]++;
				}
				toy.num[i][j]--;

				for(int k = 0; k < 3; k++) {
					toy.num[i][k] %= 4;
					toy.num[k][j] %= 4;
				}

				if(seen_safes.find(toy) == seen_safes.end()) {
					seen_safes.insert(toy);
					pq.push(make_pair(toy, current_safe.second + 1));
				}
			}
		}
	}

	if(current_safe.first.solved() == true) {
		cout << current_safe.second << endl;
	} else {
		cout << "-1" << endl;
	}

	return 0;
}