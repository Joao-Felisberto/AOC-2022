#include <fstream>
#include <iostream>
#include <queue>
#include <string>

#include "./game.cpp"

using namespace std;

int main() {
  fstream newfile;
  newfile.open("../input.txt", ios::in);

  if (!newfile.is_open()) {
    return -1;
  }

  int result = 0;
  string line;
  while (getline(newfile, line)) {
    Choice adversary = fromChar(line[0]);
    char outcome = line[2];

    Choice mine = play(adversary, outcome);
    result += playAgainst(mine, adversary);
  }
  newfile.close();

  cout << result << endl;

  return 0;
}