#include <iostream>
#include <algorithm>
#include <limits>
#include <queue>

#define MAX_NODES 10000

using namespace std;

int graph[MAX_NODES][MAX_NODES];
int dist[MAX_NODES];

struct Vertex {
	int v;
	int w;

	Vertex() {
		v = -1;
		w = -1;
	}

	Vertex(int a, int b) {
		v = a;
		w = b;
	}
};

struct MyComparison {
	bool operator()(const Vertex& a, const Vertex& b) {
		return a.w > b.w;
	}
};

void dijkstra(const int source, const int nodes) {
	dist[source] = 0;

	priority_queue<Vertex, vector<Vertex>, MyComparison> pq;

	Vertex vert;
	int alt;

	for(int i = 0; i < nodes; i++) {
		if(i != source) {
			dist[i] = numeric_limits<int>::max();
		}
		pq.push(Vertex(i, dist[i]));
	}

	while(!pq.empty()) {
		vert = pq.top();
		pq.pop();

		for(int i = 0; i < nodes; i++) {
			if(graph[vert.v][i] != -1) {
				alt = dist[vert.v] + graph[vert.v][i];

				if(alt < dist[i] && alt >= 0) {
					dist[i] = alt;
					pq.push(Vertex(i, alt));
				}
			}
		}
	}

	return;
}


int main() {
	int nodes, edges, queries, starting_node;
	int v, u, w;

	while(true) {
		cin >> nodes >> edges >> queries >> starting_node;

		if(nodes == 0) {
			break;
		}

		for(int i = 0; i < nodes; i++) {
			fill(graph[i], graph[i]+nodes, -1);
		}

		for(int i = 0; i < edges; i++) {
			cin >> v >> u >> w;
			graph[v][u] = w;
		}

		dijkstra(starting_node, nodes);

		
		for(int i = 0; i < queries; i++) {
			cin >> v;

			if(dist[v] == numeric_limits<int>::max()) {
				cout << "Impossible" << endl;
			} else {
				cout << dist[v] << endl;
			}
		}
		

		cout << endl;
	}

	return 0;
}