#include <iostream>
#include <vector>
using namespace std;

void GreedyTour(vector<vector<double> > podoubles);

double dist(vector<double> a, vector<double> b) {
	return ((a[0] - b[0]) * (a[0] - b[0])) + ((a[1] - b[1]) * (a[1] - b[1]));
}

int main() {
	double podoubles;
	double x, y;
	vector<vector<double> > coor;
	vector<double> temp;

	cin >> podoubles;

	for(double i = 0; i < podoubles; i++) {
		cin >> x >> y;
		temp = vector<double>();
		temp.push_back(x);
		temp.push_back(y);
		coor.push_back(temp);
	}

	GreedyTour(coor);

	return 0;
}

void GreedyTour(vector<vector<double> > podoubles) {
	vector<double> tour(podoubles.size(), 0);
	vector<bool> used(podoubles.size(), false);
	used[0] = true;
	double best;
	double n = podoubles.size();

	for(double i = 1; i < n; i++) {
		best = -1;
		for(double j = 0; j < n; j++) {
			if(!used[j] and (best == -1 || 
				dist(podoubles[tour[i-1]], podoubles[j]) < dist(podoubles[tour[i-1]], podoubles[best]))) {
				best = j;
			}
		}
		tour[i] = best;
		used[best] = true;
	}

	for(auto i : tour) {
		cout << i << endl;
	}

	return;
}