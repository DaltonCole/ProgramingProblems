#include <iostream>
#include <vector>
#include <queue>
using namespace std;

void prim(int nodes, int edges);

int main() {
	int nodes, edges;
	int to, from, weight;

	while(cin >> nodes >> edges) {
		if(nodes == 0 and edges == 0) {
			break;
		}

		prim(nodes, edges);
	}
}

// Used to store the vertex number and the current cost to get to that vertex
struct Vertex {
	int number;
	int weight;

	Vertex() {
		number = 0;
		weight = 0;
	}

	Vertex(int n, int w) {
		number = n;
		weight = w;
	}

	Vertex & operator = (const Vertex &rhs) {
		number = rhs.number;
		weight = rhs.weight;

		return *this;
	}
};

// Compare function class for organizing the priority queue
class Compare {
	public:
		bool operator() (Vertex a, Vertex b) {
			return (a.weight > b.weight ? true : false);
		}
};


/*
-------------- MST-PRIM(G, w, r) --------------
for each u in G.V
	u.key = infinity
	u.source = NIL
r.key = 0
Q = G.V
while Q is not Empty
	u = Extract-Min(Q)
	for each v in G.Adj[u]
		if v in Q and w(u,v) < v.key
			v.source = u
			v.key = w(u,v)
*/
void prim(int nodes, int edges) {
	int INFINITY = 999; 	// What infinity is (greater than any possible shortest path length)
	int source = 0;			// Source vertex

	// Adjacency_matrix
	vector<vector<int> > adjacency_matrix;

	// Size of square matrix
	int size;

	// Priority Queue
	priority_queue<Vertex, std::vector<Vertex>, Compare> pq;

	// vertex - Used to store poped element from priority queue
	// vertex2 - Used as place holder to push new elements into the priority queue
	Vertex vertex, vertex2;

	// Vector distances
	vector<int> distances;

	// Path trace
	vector<int> trace;

	// Visited
	vector<bool> visited;

	// Total weight
	int total = 0;

	cin >> size;		// Read in size
	vector<int> column; // Column vector

	// Initialize vectors
	for(int i = 0; i < size; i++) {
		// Initialize columns in adjacency matrix
		column.push_back(0);
		// Infinite distance from source to destination for all verticies except source to source
		distances.push_back(INFINITY);
		// Initialize trace from i to i
		trace.push_back(INFINITY);
		// Initialize visted to false for all vertices
		visited.push_back(false);
	}

	// Set distance from the source to the source to 0
	distances[source] = 0;
	trace[source] = source;

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

	// Add vertecies to queue
	for(int i = 0; i < size; i++) {
		vertex.number = i;
		vertex.weight = distances[i];
		pq.push(vertex);
	}

	// While pq is not empty
	while(pq.empty() != true) {
		// Pop from priority queue
		vertex = pq.top();
		pq.pop();
		// If we've already visited the vertex, ignore it
		if(visited[vertex.number] == false) {
			// Mark vertex as visited
			visited[vertex.number] = true;
			// For each adjacent vertex
			for(int i = 0; i < size; i++) {
				// See if edge between vertex and i exists
				if(adjacency_matrix[vertex.number][i] != 0 && adjacency_matrix[vertex.number][i] != INFINITY) {
					// If vertex has not been visited and weight(u,v) < v.key
					if(visited[i] == false && adjacency_matrix[vertex.number][i] < distances[i]) {
						// Update path
						trace[i] = vertex.number;
						// Update distance
						distances[i] = adjacency_matrix[vertex.number][i];
						// Update vertex v in priority queue
						vertex2.number = i;
						vertex2.weight = distances[i];
						pq.push(vertex2);
					}
				}
			}
		}
	}

	// Print shortest distance and path
	cout << "Distances:" << endl;
	for(int i = 0; i < size; i++) {
		cout << i << "\t" << distances[i] << endl;
		total += distances[i];
	}

	cout << endl << "Trace:" << endl;
	for(int i = 0; i < size; i++) {
		cout << i << "\t" << trace[i] << endl;
	}

	// Print Total weight of graph
	cout << endl << "Total weight:" << endl << total << endl;

	return;
}