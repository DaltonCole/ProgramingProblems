#include <iostream>
#include <queue>
#include <vector>
#include <functional>

using namespace std;

int main() {
	int cases;
	int g_army_size;
	int m_army_size;
	int temp;
	int min_g, min_m;

	priority_queue<int, vector<int> > godzilla;
	priority_queue<int, vector<int> > mechagodzilla;

	cin >> cases;

	for(int i = 0; i < cases; i++) {
		cin >> g_army_size >> m_army_size;
		for(int j = 0; j < g_army_size; j++) {
			cin >> temp;
			godzilla.push(temp);
		} 
		for(int j = 0; j < m_army_size; j++) {
			cin >> temp;
			mechagodzilla.push(temp);
		}

		while(godzilla.size() != 0 && mechagodzilla.size() != 0) {
			min_g = godzilla.top();
			min_m = mechagodzilla.top();
			(min_g < min_m)? godzilla.pop() : mechagodzilla.pop();
		}

		if(mechagodzilla.size() == 0) {
			cout << "Godzilla" << endl;
		} else {
			cout << "MechaGodzilla" << endl;
		}

		// Clear priority queues
		godzilla = priority_queue<int, vector<int> >();
		mechagodzilla = priority_queue<int, vector<int> >();
	}

	return 0;
}