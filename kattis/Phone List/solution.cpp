#include <algorithm>
#include <iostream>
#include <string>
#include <unordered_set>
#include <memory>

using namespace std;

struct Trie {
	bool accept_state;
	char number;
	unordered_set<shared_ptr<Trie> > next_nodes;

	Trie(): accept_state(false), number('-') {}

	Trie(bool acc, char num): accept_state(acc), number(num) {}

	bool operator==(const Trie& other) {return number == other.number ? true : false;}

	bool add_word(string& word) {
		if(word.size() == 0 && next_nodes.size() != 0) {
			return false;
		}

		// Base case - Word size is 0
		if(word.size() == 0) {
			accept_state = true;
			return true;
		}

		// If more word to go, and in an accept state
		if(accept_state == true) {
			return false;
		}

		// Make node
		char num = word.back();
		shared_ptr<Trie> node(new Trie(false, num));
		word.pop_back();

		for(auto& n : next_nodes) {
			if(n -> number == num) {
				return n -> add_word(word);
			}
		}

		// Add node and explore
		next_nodes.insert(node);
		return node -> add_word(word);
	}
};

bool operator==(const Trie& lhs, const Trie& rhs) {return lhs.number == rhs.number ? true : false;}

/*
namespace std {
	template <>
	struct hash<shared_ptr<Trie> > {
		std::size_t operator()(const shared_ptr<Trie>& t) const {
			return hash<char>()(t -> number);
		}
	};
}
*/

int main() {
	int cases;
	int inputs;
	string number;
	bool valid;

	cin >> cases;

	for(int a = 0; a < cases; a++) {
		cin >> inputs;
		valid = true;

		Trie prefix_tree;

		for(int b = 0; b < inputs; b++) {
			cin >> number;

			// Reverse string so we can do pop_back
			reverse(number.begin(), number.end());

			if(valid) {
				if(prefix_tree.add_word(number) == false) {
					valid = false;
				}
			}
		}

		valid == true ? cout << "YES" << endl : cout << "NO" << endl;
	}

	return 0;
}