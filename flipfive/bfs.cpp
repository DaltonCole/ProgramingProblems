#include <iostream>
#include <queue>
#include <algorithm>
#include <memory>
#include <unordered_set>

using namespace std;

// Do BFS!

struct Packet {
	bool grid[3][3];
	int depth;

	Packet() {
		for(int i = 0; i < 3; i++) {
			for(int j = 0; j < 3; j++) {
				grid[i][j] = false;
			}
		}
		depth = 0;
	}

	void fill() {
		string line;
		depth = 0;
		for(int i = 0; i < 3; i++) {
			cin >> line;
			for(int j = 0; j < 3; j++) {
				if(line[j] == '.') {
					grid[i][j] = true;
				} else {
					grid[i][j] = false;
				}
			}
		}
	}

	void print() {
		cout << depth << endl;
		for(int i = 0; i < 3; i++) {
			for(int j = 0; j < 3; j++) {
				cout << grid[i][j];
			}
			cout << endl;
		}
		cout << endl;
	}

	bool flip(const int x, const int y) {
		depth++;

		grid[x][y] = !grid[x][y];
		if(x > 0) {
			grid[x-1][y] = !grid[x-1][y];
		}
		if(y > 0) {
			grid[x][y-1] = !grid[x][y-1];
		}
		if(x < 2) {
			grid[x+1][y] = !grid[x+1][y];
		}
		if(y < 2) {
			grid[x][y+1] = !grid[x][y+1];
		}

		return check();
	}

	bool check() {
		for(int i = 0; i < 3; i++) {
			for(int j = 0; j < 3; j++) {
				if(grid[i][j] == false) {
					return false;
				}
			}
		}
		return true;
	}

	bool operator==(const Packet& rhs) {
		for(int i = 0; i < 3; i++) {
			for(int j = 0; j < 3; j++) {
				if(grid[i][j] != rhs.grid[i][j]) {
					return false;
				}
			}
		}
		return true;
	}
};

bool operator==(const Packet& lhs, const Packet& rhs) {
	for(int i = 0; i < 3; i++) {
		for(int j = 0; j < 3; j++) {
			if(lhs.grid[i][j] != rhs.grid[i][j]) {
				return false;
			}
		}
	}
	return true;
}

namespace std {
	template<>
	struct hash<Packet> {
		size_t operator()(const Packet& obj) const {
			return hash<bool>()(obj.grid);
		}
	};
}

int main() {
	int cases;
	string line;
	Packet temp;
	Packet other;
	queue<Packet> q;
	unordered_set<Packet> s;
	bool done = false;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		temp.fill();
		q = queue<Packet>();
		s.clear();

		q.push(temp);
		s.insert(temp);

		other = temp;
		done = other.check();

		while(!done) {
			temp = q.front();
			q.pop();

			for(int x = 0; x < 3 && !done; x++) {
				for(int y = 0; y < 3 && !done; y++) {
					other = temp;
					done = other.flip(x, y);
					if(s.find(other) == s.end()) {
						q.push(other);
						s.insert(other);
					}
				}
			}
		}

		cout << other.depth << endl;
	}

	return 0;
}