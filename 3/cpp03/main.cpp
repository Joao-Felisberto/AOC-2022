#include <fstream>
#include <iostream>
#include <ostream>
#include <queue>
#include <stdio.h>
#include <string>

using namespace std;

int domainHash(int e) {
  if (e >= 65 && e <= 90) {
    // upper
    return e - (65 - 27) - 1;
  } else if (e <= 122) {
    // lower
    return e - 96 - 1;
  }
  return -1; // error, should not happen
}

int main() {
  int badgeW = 0;
  int badges_hash[52];
  for (int i = 0; i < sizeof(badges_hash); badges_hash[i++] = 0)
    ;
  int mod3_cnt = 0;

  int result = 0;

  fstream newfile;
  newfile.open("../input.txt", ios::in);

  if (!newfile.is_open()) {
    return -1;
  }
  string line;

  while (getline(newfile, line)) {
    int j = 0;
    int half = (sizeof(line) - 1) / 2;

    bool left_hash[52];
    for (int i = 0; i < sizeof(left_hash); left_hash[i++] = false)
      ;
    bool right_hash[52];
    for (int i = 0; i < sizeof(right_hash); right_hash[i++] = false)
      ;

    for (int i = 0; i < half; i++) {
      cout << "HERE " 
           << sizeof(line) << " " 
           << i << " "
           << i + half
           << endl;
      int left = domainHash(line[i]);
      int right = domainHash(line[i + half]);
      cout << left << " " << right << " " << sizeof(left_hash) << " " << sizeof(right_hash) << endl;

      left_hash[left] = true;
      right_hash[right] = true;

      if (right_hash[left]) {
        result += left + 1;
        j = i;
        break;
      }
      cout << "IT " << left_hash[right] << endl;
      if (left_hash[right]) {
        result += right + 1;
        j = i;
        break;
      }
      cout << "BROKEN" << endl;
      //cout << right_hash[left] << " " << left_hash[right] << endl;
    }

    for (int i = j; i < half; i++) {
      int left = domainHash(line[i]);
      int right = domainHash(line[i + half]);

      left_hash[left] = true;
      right_hash[right] = true;
    }

    for (int i = 0; i < sizeof(badges_hash); i++) {
      if (left_hash[i] || right_hash[i]) {
        badges_hash[i] += 1;
      }
    }

    if (mod3_cnt == 2) {
      for (int i = 0; i < sizeof(badges_hash); i++) {
        if (badges_hash[i] == 3) {
          badgeW += i + 1;
          break;
        }
      }
      for (int i = 0; i < sizeof(badges_hash); badges_hash[i++] = 0)
        ;
    }
    mod3_cnt = (mod3_cnt + 1) % 3;
  }
  newfile.close();

  cout << result << endl;
  cout << badgeW << endl;

  return 0;
}