from collections import namedtuple
from enum import Enum

class TokenTypes(Enum):
	UNDEFINED = 0
	NUMBER = 1
	STRING = 2
	IDENTIFIER = 3
	OPEN_PAREN = 4
	CLOSE_PAREN = 5

	COMMENT = 6
	MULTILINE_COMMENT = 7

Token = namedtuple("Token", "type data index")