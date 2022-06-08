#!/home/codespace/.python/current/bin/python3

def tokenize(program):
    return program.replace('(', ' ( ').replace(')', ' ) ').split()

def parse(program):
    return tokenize(program)

def main():
    while True:
        program = input("crisp >> ")
        if program == "(quit)":
            return

        parsed = parse(program)

        print("      >> " + " ".join(parsed))

if __name__ == "__main__":
    main()
