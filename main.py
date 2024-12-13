import sys
import subprocess
import pyperclip

f = open(".zsh_history")
lines = f.readlines()

def func(u):
    matches = [line for line in lines if u in line]
    return matches

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python main.py <snippet>")
        sys.exit(1)
    snippet = sys.argv[1]
    results = func(snippet)
    if results:
        if len(results) == 1:
            command = results[0]
            print(command)
            pyperclip.copy(command)
            print("Command copied to clipboard. Paste it into the terminal to execute.")
        else:
            print("Multiple matches found:")
            for i, result in enumerate(results):
                print(f"{i + 1}: {result.strip()}")
            choice = int(input("Enter the number of the command you want: ")) - 1
            if 0 <= choice < len(results):
                command = results[choice]
                print(command)
                pyperclip.copy(command)
                print("Command copied to clipboard. Paste it into the terminal to execute.")
            else:
                print("Invalid choice.")
    else:
        print("No matching command found.")
