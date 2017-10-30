#include <iostream>
#include <limits>
#include <vector>
#include <queue>
#include <stack>
#include <unordered_map>
#include <unordered_set>
#include <algorithm>

using namespace std;

void kruskal(int size, int edges);


int main() {
	int nodes, edges;
	int to, from, weight;

	while(cin >> nodes >> edges) {
		if(nodes == 0 and edges == 0) {
			break;
		}

		kruskal(nodes, edges);
	}
}

// Used to store each edge in the graph
struct Edge {
	int source;
	int destination;
	int weight;

	Edge() {
		source = 0;
		destination = 0;
		weight = 0;
	}

	Edge(int s, int d, int w) {
		source = s;
		destination = d;
		weight = w;
	}

	Edge & operator = (const Edge &rhs) {
		source = rhs.source;
		destination = rhs.destination;
		weight = rhs.weight;

		return *this;
	}
};

// Compare function class for organizing the priority queue
class Compare {
	public:
		bool operator() (Edge a, Edge b) {
			return (a.weight > b.weight ? true : false);
		}
};


bool order(Edge a, Edge b) {
	if(a.source < b.source) {
		return true;
	} else if(a.source == b.source) {
		return (a.destination < b.destination ? true : false);
	} else {
		return false;
	}
}

void kruskal(int size, int edges) {
	// Priority Queue - Used to sort edges from lowest weight to highest
	priority_queue<Edge, std::vector<Edge>, Compare> pq;

	// Used to add and pop edges to/from priority queue
	Edge edge1;

	// Vector to contain all of the sets
	vector<int> set;

	// Edges of final mst tree
	vector<Edge> mst_edges;

	// Number for destination set
	int destination_set;

	int to, from, weight;

	// Make sets
	for(int i = 0; i < size; i++) {
		set.push_back(i);
	}

	// Add Edges to queue
	// We only need the upper triangular matrix since we expect an undirected graph
	for(int i = 0; i < edges; i++) {
		cin >> to >> from >> weight;
		edge1.source = to;
		edge1.destination = from;
		edge1.weight = weight;
		pq.push(edge1);
		edge1.source = from;
		edge1.destination = to;
		pq.push(edge1);
	}


	// While there are still edges
	while(pq.empty() == false) {
		// Pop lightest edge off of queue
		edge1 = pq.top();
		pq.pop();
		// If source and destination are NOT in the same set
		if(set[edge1.source] != set[edge1.destination]) {
			// Add edge to the mst edge vector
			mst_edges.push_back(edge1);
			// Combine set v into set u
			destination_set = set[edge1.destination];
			for(int i = 0; i < size; i++) {
				if(set[i] == destination_set) {
					set[i] = set[edge1.source];
				}
			}
		}
	}

	if(mst_edges.size() != size - 1) {
		cout << "Impossible" << endl;
		return;
	}


	// Sort by source, destination for printing
	sort(mst_edges.begin(), mst_edges.end(), order);
	int total_weight = 0;

	for(auto edge : mst_edges) {
		total_weight += edge.weight;
	}

	cout << total_weight << endl;

	for(auto i : mst_edges) {
		cout << i.source << " " << i.destination << endl;
	}


	return;
}