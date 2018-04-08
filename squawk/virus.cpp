#include <iostream>
#include <array>

using namespace std;

//std::array<int,5> first = {10, 20, 30, 40, 50};


bool connections[100][100];
//int original[100];
//int proprogate[100];

array<long, 100> original;
array<long, 100> proprogate;

int main() {
	int users, links, initial, time;
	int x, y;

	cin >> users >> links >> initial >> time;

	// If no time, only one squak
	if(time == 0) {
		cout << "1" << endl;
		return 0;
	}

	for(int i = 0; i < links; i++) {
		cin >> x >> y;
		connections[x][y] = true;
		connections[y][x] = true;
	}

	// 1
	for(int i = 0; i < users; i++) {
		// If there is a connection
		if(connections[initial][i] == true) {
			original[i]++;
		}
	}

	for(int i = 2; i <= time; i++) {
		for(int person = 0; person < users; person++) {
			if(original[person] != 0) {
				for(int con = 0; con < users; con++) {
					if(connections[person][con] == true) {
						proprogate[con] += original[person];
					}
				}
			}
		}

		original.swap(proprogate);
		proprogate.fill(0);
	}

	long count = 0;
	for(int i = 0; i < users; i++) {
		count += original[i];
	}

	cout << count << endl;

	return 0;
}