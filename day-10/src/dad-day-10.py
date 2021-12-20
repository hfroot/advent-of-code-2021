# Algorithm designed by Dad

def main():
    score = 0
    # f = open("../test-input.txt","r")
    f = open("../input.txt","r")
    input = f.readlines()
    for original_line in input:
        line = original_line
        previous_line_len = 0
        while len(line) != previous_line_len:
            previous_line_len = len(line)
            idx = 0
            while idx < len(line):
                if idx + 1 >= len(line):
                    break
                char = line[idx]
                if characters_match(char, line[idx+1]):
                    if idx + 2 < len(line):
                        line = line[:idx] + line[idx+2:]
                    else:
                        line = line[:idx]
                idx += 1
        invalid = find_first_closing(line)
        if invalid == ")":
            score += 3
        elif invalid == "]":
            score += 57
        elif invalid == "}":
            score += 1197
        elif invalid == ">":
            score += 25137
    print(score)


def characters_match(opening, closing):
    if (
        (opening == "{" and closing == "}") or
        (opening == "(" and closing == ")") or
        (opening == "[" and closing == "]") or
        (opening == "<" and closing == ">")
    ):
        return True
    return False

def find_first_closing(line):
    for char in line:
        if char == ")" or char == "]" or char == "}" or char == ">":
            return char

main()
