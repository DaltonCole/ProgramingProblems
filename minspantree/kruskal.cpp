#include <iostream>
#include <vector>
#include <queue>
using namespace std;

void kruskal();

int main() {
	kruskal();

	return 0;
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

/*
-------------- MST-KRUSKAL(G,w) --------------
A = NONE
for each vertex v in G.V
	MAKE-SET(v)
sort the edges of G.E into nondecreasing order by weight w
for each edge (u,v) in G.E, taken in nondecreasing order by weight
	if Find-Set(u) != Find-Set(v)
		A = A UNION {(u,v)}
		UNION(u,v)
return A
*/
void kruskal() {
	int INFINITY = 999; 	// What infinity is (greater than any possible shortest path length)

	// Adjacency_matrix
	vector<vector<int> > adjacency_matrix;

	// Size of square matrix
	int size;

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

	cin >> size;		// Read in size
	vector<int> column; // Column vector


	// Initialize vectors
	for(int i = 0; i < size; i++) {
		// Initialize columns in adjacency matrix
		column.push_back(0);
	}

	// Set columns in matrix
	for(int i = 0; i < size; i++) {
		adjacency_matrix.push_back(column);
	}

	// Read in Adjacency Matrix
	for(int i = 0; i < size; i++) {
		for(int j = 0; j < size; j++) {
			cin >> adjacency_matrix[i][j];
		}
	}

	// Make sets
	for(int i = 0; i < size; i++) {
		set.push_back(i);
	}

	// Add Edges to queue
	// We only need the upper triangular matrix since we expect an undirected graph
	for(int i = 0; i < size; i++) {
		for(int j = i; j < size; j++) {
			if(adjacency_matrix[i][j] != INFINITY && adjacency_matrix[i][j] != 0) {
				edge1.source = i;
				edge1.destination = j;
				edge1.weight = adjacency_matrix[i][j];
				pq.push(edge1);
			}
		}
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

	// Print edges
	int total = 0;
	cout << "Source | Destination | Weight" << endl;
	for(int i = 0; i < mst_edges.size(); i++) {
		cout << mst_edges[i].source << " \t" << mst_edges[i].destination << " \t" << mst_edges[i].weight << endl;
		total += mst_edges[i].weight;
	}

	cout << endl << "Total weight:" << endl;
	cout << total << endl;

	return;
}