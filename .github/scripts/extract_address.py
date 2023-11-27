import sys
import re

def extract_address(issue_body):
    # Regular expression pattern to match the address following 'Address: '
    pattern = r'### Public Address\s*\n([A-Za-z0-9]+)'
    match = re.search(pattern, issue_body)

    if match:
        return match.group(1)  # Return the matched address
    return "No address found"

if __name__ == "__main__":
    issue_body = sys.argv[1]
    address = extract_address(issue_body)
    print(f"::set-output name=address::{address}")
