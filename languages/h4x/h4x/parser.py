from pprint import pprint

from . import error
from . import tokens

null = tokens.Token(tokens.TokenTypes.IDENTIFIER, "", 0)

def is_valid_program(tokens_list):
	return parse(tokens_list, validate=True)

def parse(tokens_list, depth=0, validate=False):
	tree = []
	i = 0
	while i < len(tokens_list):
		token = tokens_list[i]
		value = token
		if token.type == tokens.TokenTypes.OPEN_PAREN:
			match_index = get_matching(i, tokens_list, validate)
			if match_index == False:
				return False
			value = parse(tokens_list[i+1 : match_index], depth+1)
			i = match_index
		tree.append(value)
		i += 1
	if validate:
		return True
	return tree

def get_matching(start, tokens_list, validate):
	depth = 0
	index = start
	while depth >= 0:
		index += 1
		if index > len(tokens_list) - 1:
			if validate:
				return False
			error.parser("Unmatched parenthesis", tokens_list[start])
		token = tokens_list[index]
		if token.type == tokens.TokenTypes.OPEN_PAREN:
			depth += 1
		elif token.type == tokens.TokenTypes.CLOSE_PAREN:
			depth -= 1
	return index
