/*
# Resources used

- htlines://www.tutorialspoint.com/read-file-line-by-line-using-cplusplus
- https://www.geeksforgeeks.org/priority-queue-in-cpp-stl/
- https://www.freecodecamp.org/news/string-to-int-in-c-how-to-convert-a-string-to-an-integer-example/
*/

#include <fstream>
#include <iostream>
#include <queue>
#include <string>
using namespace std;

int main() {
  priority_queue<int> elves;
  elves.push(0);

  fstream newfile;
  newfile.open("../input.txt", ios::in);

  if (!newfile.is_open()) {
    return -1;
  }

  int calories = 0;
  string line;
  while (getline(newfile, line)) {
    if (line == string("")) {
      elves.push(calories);
      calories = 0;
    } else {
      calories += stoi(line);
    }
  }
  newfile.close();

  int top1 = elves.top(); elves.pop();
  int top2 = elves.top(); elves.pop();
  int top3 = elves.top(); elves.pop();

  int sum = top1 + top2 + top3;
  cout << "top 3 elves carry " << sum << " calories" << endl;
  return 0;
}