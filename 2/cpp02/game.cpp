/*
# Resources

- https://www.tutorialspoint.com/how-to-use-enums-in-cplusplus
- https://stackoverflow.com/questions/2571456/operators-overloading-for-enums
-
https://stackoverflow.com/questions/18335861/why-is-enum-class-preferred-over-plain-enum
- https://www.w3schools.com/cpp/cpp_switch.asp
*/

#include <cstddef>
#include <iostream>
#include <ostream>
enum class Choice {
  Rock = 1,
  Paper = 2,
  Scisors = 3,
};

Choice fromChar(char c) {
  if (c == 'A' || c == 'X')
    return Choice::Rock;
  if (c == 'B' || c == 'Y')
    return Choice::Paper;
  if (c == 'C' || c == 'Z')
    return Choice::Scisors;

  // should never happen, make this memory safe
  return Choice::Paper;
}

Choice play(Choice adversary, char result) {
  switch (result) {
  case 'X':
    switch (adversary) {
    case Choice::Rock:
      return Choice::Scisors;
    case Choice::Paper:
      return Choice::Rock;
    case Choice::Scisors:
      return Choice::Paper;
    }
    break;
  case 'Y':
    return adversary;
    break;
  case 'Z':
    switch (adversary) {
    case Choice::Rock:
      return Choice::Paper;
    case Choice::Paper:
      return Choice::Scisors;
    case Choice::Scisors:
      return Choice::Rock;
    }
    break;
  }

  // should never run
  std::cout << "ERR" << std::endl;
  return Choice::Rock;
}

int playAgainst(Choice player, Choice adversary) {
  int result;

  if (player > adversary)
    result = 6;
  else if (player == adversary)
    result = 3;
  else
    result = 0;

  return (int)player + result;
}


bool operator<(Choice lhs, Choice rhs) {
  return lhs == Choice::Rock && rhs == Choice::Paper ||
         lhs == Choice::Paper && rhs == Choice::Scisors ||
         lhs == Choice::Scisors && rhs == Choice::Rock;
}

bool operator>(Choice lhs, Choice rhs) {
  return lhs == Choice::Paper && rhs == Choice::Rock ||
         lhs == Choice::Scisors && rhs == Choice::Paper ||
         lhs == Choice::Rock && rhs == Choice::Scisors;
}
