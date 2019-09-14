#include <iostream>
#include <unordered_map>
#include <map>
#include <string>
#include <sstream>
#include <vector>
#include <iterator>
#include <queue>
#include <utility>
#include <unordered_set>

using namespace std;

int MAX_VALUE = -1;

template<typename Out>
void split(const std::string &s, char delim, Out result) {
    std::stringstream ss(s);
    std::string item;
    while (std::getline(ss, item, delim)) {
        *(result++) = item;
    }
}

std::vector<std::string> split(const std::string &s, char delim) {
    std::vector<std::string> elems;
    split(s, delim, std::back_inserter(elems));
    return elems;
}

struct Node {
	string name;
	int distance;

	Node(string n, int d): name(n), distance(d){}
};

struct myCompare {
	bool operator()(const Node& lhs, const Node& rhs) {
		return lhs.distance < rhs.distance;
	}
};

int main() {
	int rules;
	string line;
	string start;
	unordered_map<string, unordered_set<string> > edges;
	unordered_map<string, int> distances;
	priority_queue<Node, vector<Node>, myCompare> q;


	cin >> rules;
	getline(cin, line); // Get rid of newline

	for(int i = 0; i < rules; i++) {
		// Get line
		getline(cin, line);

		// Split by space
		auto split_string = split(line, ' ');
		// Remove ':' from first string
		split_string[0].pop_back();

		// Add outgoing edges
		for(int i = 1; i < split_string.size(); i++) {
			edges[split_string[i]].insert(split_string[0]);
		}

		// Distances
		distances[split_string[0]] = MAX_VALUE;
	}

	cin >> start;

	// Dijkstra's
	distances[start] = 0;
	unordered_set<string> looked_at;

	q.emplace(start, 0);

	while(q.empty() == false) {
		Node top = q.top();
		q.pop();

		looked_at.insert(top.name);

		for(const auto& neighbor_name : edges[top.name]) {
			if(distances[top.name] + 1 > distances[neighbor_name]) {
				distances[neighbor_name] = distances[top.name] + 1;
			}

			if(looked_at.find(neighbor_name) == looked_at.end()) {
				q.emplace(neighbor_name, distances[neighbor_name]);
			}
		}
	}

	// Use map to order, reverse keys and values
	multimap<int, string> result;

	for(auto& dist : distances) {
		if(dist.second != MAX_VALUE) {
			result.emplace(dist.second, dist.first);
		}
	}

	for(const auto& res : result) {
		cout << res.second << endl;
	}


	return 0;
}