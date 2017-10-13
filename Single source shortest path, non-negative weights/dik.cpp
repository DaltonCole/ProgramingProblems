#include <iostream>
#include <vector>
#include <queue>
using namespace std;

void dijkstra();

int main() {
	dijkstra();

	return 0;
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
-------------- Dijkstra's Algorithm --------------
INITIALIZE-SINGLE-SOURCE(G,s)
S = NONE
Q = G.V
while Q != 0
	u = EXTRACT-MIN(Q)
	S = S UNION {u}
	for each vertex v in G.Adj[u]
		if v.d > u.d + w(u,v)
			v.d = u.d + w(u,v)
			v.source = u
*/
void dijkstra() {
	int INFINITY = 999; 	// What infinity is (greater than any possible shortest path length)
	int source = 1;			// Source vertex

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

	cin >> size;		// Read in size
	vector<int> column; // Column vector

	// Initialize vectors
	for(int i = 0; i < size; i++) {
		// Initialize columns in adjacency matrix
		column.push_back(0);
		// Infinite distance from source to destination for all verticies except source to source
		distances.push_back(INFINITY);
		// Initialize trace from i to i
		trace.push_back(i);
		// Initialize visted to false for all vertices
		visited.push_back(false);
	}

	// Set distance from the source to the source to 0
	distances[source] = 0;

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
			// For each possible edge
			for(int i = 0; i < size; i++) {
				// See if edge between vertex and i exists
				if(adjacency_matrix[vertex.number][i] != 0 && adjacency_matrix[vertex.number][i] != INFINITY && visited[i] == false) {
					// See if updating the distance would improve the distance
					if(distances[i] > distances[vertex.number] + adjacency_matrix[vertex.number][i]) {
						// Update distance vector
						distances[i] = distances[vertex.number] + adjacency_matrix[vertex.number][i];
						// Update trace vector
						trace[i] = vertex.number;
						// Add new weight and vertex number to priority queue
						vertex2.weight = distances[i];
						vertex2.number = i;
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
	}

	cout << endl << "Trace:" << endl;
	for(int i = 0; i < size; i++) {
		cout << i << "\t" << trace[i] << endl;
	}

	return;
}