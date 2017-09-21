#include <iostream>
#include <vector>
#include <unordered_set>
#include <stack>

using namespace std;

struct Boogle{
	Boogle() {
		for(int i = 0; i < 4; i++) {
			board.push_back(vector<bool>(4, false));
		}
		word = "";
	}

	// Word
	string word;
	vector<vector<bool> > board;
	int last_x;
	int last_y;
};

int find_score(const string & word) {
	if(word.length() == 3 or word.length() == 4) {
		return 1;
	}
	if(word.length() == 5) {
		return 2;
	}
	if(word.length() == 6) {
		return 3;
	}
	if(word.length() == 7) {
		return 5;
	}
	if(word.length() == 8) {
		return 11;
	}
}



int main() {
	// Number of words and boards
	int words;
	// Score for the board
	int score;
	// Best word on the board
	string best_word;
	// Extra string
	string word_input;
	// The dinctionary
	unordered_set<string> dictionary;
	// Contains all of the subwords that could be made
	unordered_set<string> sub_dictionary;
	// Find iterator for unordered_set
	std::unordered_set<std::string>::const_iterator got;
	// Set of found words
	unordered_set<string> found_words;

	// Stack
	stack<Boogle> s;

	// The boogle board and a temp
	Boogle grid;
	Boogle temp;


	// Initalize board
	vector<vector<char> > board;
	for(int i = 0; i < 4; i++) {
		board.push_back(vector<char>(4, '-'));
	}
	// Second board
	vector<vector<char> > board2;

	// Number of words in dictionary
	cin >> words;

	// Add words to the dictionary
	for(int i = 0; i < words; i++) {
		cin >> word_input;
		dictionary.insert(word_input);
	}

	// Create a sub dictionary
	for(auto i : dictionary) {
		word_input = "";
		for(auto c : i) {
			word_input += c;
			sub_dictionary.insert(word_input);
		}
	}

	// Number of boards
	cin >> words;

	for(int i = 0; i < words; i++) {
		// Get blank line
		getline(cin, word_input);

		// Make board
		for(int j = 0; j < 4; j++) {
			getline(cin, word_input);
			for(int k = 0; k < 4; k++) {
				board[j][k] = word_input[k];
			}
		} 
		if(board == board2) {
			cout << score << " " << best_word << " " << found_words.size() << endl;
			continue;
		}
		found_words.clear();
		board2 = board;

		// Initalize boogle board with 16 boards
		for(int j = 0; j < 4; j++) {
			for(int k = 0; k < 4; k++) {
				// Reset Boogle board
				grid = Boogle();
				// Add True value to board
				grid.board[j][k] = true;
				// Add letter
				grid.word += board[j][k];
				// Update x and y coordinates for last letter
				grid.last_x = j;
				grid.last_y = k;
				// See if this word is a sub word of a dictionary word
				//	If so, then add to stack
				got = sub_dictionary.find(grid.word);
				if(got != sub_dictionary.end()) {
					s.push(grid);
				}	
			}
		}

		// While stack is not empty
		while(!s.empty()) {
			grid = s.top();
			s.pop();

			// See if string is in set
			got = dictionary.find(grid.word);
			if(got != dictionary.end()) {
				found_words.insert(grid.word);
			}

			// If word is 8 long go to next word
			if(grid.word.length() == 8) {
				continue;
			}

			// Add other boards //
			// Up
			if(grid.last_y < 3) {
				if(grid.board[grid.last_x][grid.last_y + 1] != true) {
					got = sub_dictionary.find(grid.word + board[grid.last_x][grid.last_y + 1]);
					if(got != sub_dictionary.end()) {
						temp = grid;
						temp.last_y += 1;
						temp.board[temp.last_x][temp.last_y] = true;
						temp.word += board[temp.last_x][temp.last_y];
						s.push(temp);
					}
				}
			}
			// Up right
			if(grid.last_y < 3 and grid.last_x < 3) {
				if(grid.board[grid.last_x + 1][grid.last_y + 1] != true) {
					got = sub_dictionary.find(grid.word + board[grid.last_x + 1][grid.last_y + 1]);
					if(got != sub_dictionary.end()) {
						temp = grid;
						temp.last_y += 1;
						temp.last_x += 1;
						temp.board[temp.last_x][temp.last_y] = true;
						temp.word += board[temp.last_x][temp.last_y];
						s.push(temp);
					}
					
				}
			}
			// Up Left
			if(grid.last_y < 3 and grid.last_x > 0) {
				if(grid.board[grid.last_x - 1][grid.last_y + 1] != true) {
					got = sub_dictionary.find(grid.word + board[grid.last_x - 1][grid.last_y + 1]);
					if(got != sub_dictionary.end()) {
						temp = grid;
						temp.last_y += 1;
						temp.last_x -= 1;
						temp.board[temp.last_x][temp.last_y] = true;
						temp.word += board[temp.last_x][temp.last_y];
						s.push(temp);
					}
				}
			}
			// Right
			if(grid.last_x < 3) {
				if(grid.board[grid.last_x + 1][grid.last_y] != true) {
					got = sub_dictionary.find(grid.word + board[grid.last_x + 1][grid.last_y]);
					if(got != sub_dictionary.end()) {
						temp = grid;
						temp.last_x += 1;
						temp.board[temp.last_x][temp.last_y] = true;
						temp.word += board[temp.last_x][temp.last_y];
						s.push(temp);
					}
				}
			}
			// Left
			if(grid.last_x > 0) {
				if(grid.board[grid.last_x - 1][grid.last_y] != true) {
					got = sub_dictionary.find(grid.word + board[grid.last_x - 1][grid.last_y]);
					if(got != sub_dictionary.end()) {
						temp = grid;
						temp.last_x -= 1;
						temp.board[temp.last_x][temp.last_y] = true;
						temp.word += board[temp.last_x][temp.last_y];
						s.push(temp);
					}
				}
			}
			// Down
			if(grid.last_y > 0) {
				if(grid.board[grid.last_x][grid.last_y - 1] != true) {
					got = sub_dictionary.find(grid.word + board[grid.last_x][grid.last_y - 1]);
					if(got != sub_dictionary.end()) {
						temp = grid;
						temp.last_y -= 1;
						temp.board[temp.last_x][temp.last_y] = true;
						temp.word += board[temp.last_x][temp.last_y];
						s.push(temp);
					}
				}
			}
			// Down right
			if(grid.last_y > 0 and grid.last_x < 3) {
				if(grid.board[grid.last_x + 1][grid.last_y - 1] != true) {
					got = sub_dictionary.find(grid.word + board[grid.last_x + 1][grid.last_y - 1]);
					if(got != sub_dictionary.end()) {
						temp = grid;
						temp.last_y -= 1;
						temp.last_x += 1;
						temp.board[temp.last_x][temp.last_y] = true;
						temp.word += board[temp.last_x][temp.last_y];
						s.push(temp);
					}
				}
			}
			// Down Left
			if(grid.last_y > 0 and grid.last_x > 0) {
				if(grid.board[grid.last_x - 1][grid.last_y - 1] != true) {
					got = sub_dictionary.find(grid.word + board[grid.last_x - 1][grid.last_y - 1]);
					if(got != sub_dictionary.end()) {
						temp = grid;
						temp.last_y -= 1;
						temp.last_x -= 1;
						temp.board[temp.last_x][temp.last_y] = true;
						temp.word += board[temp.last_x][temp.last_y];
						s.push(temp);
					}
				}
			}
		}

		score = 0;
		best_word = "";

		for(auto w : found_words) {
			score += find_score(w);
			if(best_word.length() == w.length()) {
				if(w < best_word) {
					best_word = w;
				}
			} else if(best_word.length() < w.length()) {
				best_word = w;
			}
		}

		cout << score << " " << best_word << " " << found_words.size() << endl;
	}
};