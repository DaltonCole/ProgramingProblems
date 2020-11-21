include <iostream>
#include <vector>
#include <queue>
#include <utility>
#include <unordered_set>

using namespace std;

const int MAX_INT = 2100000000;

struct myComparison {
	bool operator()(const pair<int, float>& lhs, const pair<int, float>& rhs) {
		return lhs.second > rhs.second;
	}
};

int main() {
	int locations;
	int temp;
	int edge_count;
	int total_garbage = 0;

	cin >> locations;

	vector<float> garbage;

	for(int i = 0; i < locations; i++) {
		cin >> temp;
		garbage.push_back(temp);
	}

	cin >> edge_count;

	vector<vector<float> > matrix(locations, vector<float>(locations, MAX_INT));

	int in, out;

	for(int i = 0; i < edge_count; i++) {
		cin >> in >> out >> temp;
		in--;
		out--;
		matrix[in][out] = temp;
		matrix[out][in] = temp;
	}

	for(const auto& garb : garbage) {
		total_garbage += garb;
	}
	total_garbage *= 1;

	for(int i = 0; i < garbage.size(); i++) {
		garbage[i] /= total_garbage;
	}

	for(int i = 0; i < locations; i++) {
		for(int j = 0; j < locations; j++) {
			matrix[i][j] += garbage[i];
		}
	}

	priority_queue<pair<int, float>, vector<pair<int, float> >, myComparison> q;
	vector<float> distance(locations, MAX_INT);
	unordered_set<int> looked_at;
	looked_at.insert(0);
	distance[0] = 0;

	q.push(make_pair(0, 0.0));

	while(q.empty() == false) {
		pair<int, float> node = q.top();
		q.pop();

		for(int i = 0; i < locations; i++) {
			if(distance[node.first] + matrix[node.first][i] < distance[i]) {
				distance[i] = distance[node.first] + matrix[node.first][i];
				if(looked_at.find(i) == looked_at.end()) {
					looked_at.insert(i);
					q.push(make_pair(i, distance[i]));
				}
			}
		}
	}

	if(distance[locations-1] >= MAX_INT) {
		cout << "impossible" << endl;
	} else {
		cout << distance[locations-1] << endl;

		float thing = distance[locations-1] - static_cast<int>(distance[locations-1]);

		cout << (thing * total_garbage) << endl;

	}

	return 0;
}