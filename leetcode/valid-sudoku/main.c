#include <stdio.h>

bool isValidSudoku(char** board, int boardSize, int* boardColSize) {
//bool isValidSudoku(char board[9][9], int boardSize, int* boardColSize) {
    // Rows
    //printf("%c\n", board[0][0]);
    for(int row = 0; row < boardSize; row++) {
        bool nums[9] = {false};
        for(int col = 0; col < *boardColSize; col++) {
            printf("before\n");
            //printf("%d, %d \n", row, col);
            printf("%d, %d = %c\n", row, col, board[row][col]);
            if(board[row][col] != '.') {
                printf("in\n");
                int index = board[row][col] - '1';
                if (nums[index] == true) {
                    return false;
                }
                nums[index] = true;
            }
            printf("after\n");
        }
    }

    // Cols
    for(int col = 0; col < *boardColSize; col++) {
        bool nums[9] = {false};
        for(int row = 0; row < boardSize; row++) {
            if(board[row][col] != '.') {
                int index = board[row][col] - '1';
                if (nums[index] == true) {
                    return false;
                }
                nums[index] = true;
            }
        }
    }

    // 3x3
    for(int row_set = 0; row_set < 3; row_set++) {
        for(int col_set = 0; col_set < 3; col_set++) {
            bool nums[9] = {false};
            for(int row = 0; row < 3; row++) {
                for(int col = 0; col < 3; col++) {
                    int row_offset = (row_set * 3) + row;
                    int col_offset = (col_set * 3) + col;
printf("%d, %d = %c\n", row_offset, col_offset, board[row_offset][col_offset]);
                    if(board[row][col] != '.') {
                        int index = board[row][col] - '1';
                        if (nums[index] == true) {
                            return false;
                        }
                        nums[index] = true;
            }                }
            }

        }
    }

   return true; 
}

int main() {
    char board[][9] = {{'5','3','.','.','7','.','.','.','.'},{'6','.','.','1','9','5','.','.','.'},{'.','9','8','.','.','.','.','6','.'},{'8','.','.','.','6','.','.','.','3'},{'4','.','.','8','.','3','.','.','1'},{'7','.','.','.','2','.','.','.','6'},{'.','6','.','.','.','.','2','8','.'},{'.','.','.','4','1','9','.','.','5'},{'.','.','.','.','8','.','.','7','9'}};

    printf("%c\n", board[0][0]);
    char board2[][9] = 
        {{'8','3','.','.','7','.','.','.','.'},{'6','.','.','1','9','5','.','.','.'},{'.','9','8','.','.','.','.','6','.'},{'8','.','.','.','6','.','.','.','3'},{'4','.','.','8','.','3','.','.','1'},{'7','.','.','.','2','.','.','.','6'},{'.','6','.','.','.','.','2','8','.'},{'.','.','.','4','1','9','.','.','5'},{'.','.','.','.','8','.','.','7','9'}};


    int nine[] = {9};

    bool val = isValidSudoku((char**)board, 9, nine);
    //bool val = isValidSudoku(board, 9, nine);

    printf("%d\n", val);

    bool val2 = isValidSudoku((char**)board2, 9, nine);
    //bool val2 = isValidSudoku(board2, 9, nine);
    printf("%d\n", val2);
    return 0;
}
