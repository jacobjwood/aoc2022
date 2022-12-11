
import sys

import requests


def main(day):
    
    with open("session_cookie.txt",'r') as f:
        cookie = f.readline().strip()
    
    headers = {'session': cookie}
    url = "https://adventofcode.com/2022/day/" + day + "/input"

    session = requests.Session()
    resp = session.get(url,cookies=headers)

    with open("day" + day + "/input.txt", 'w') as f:
        f.write(resp.text[:-1]) # Removes blankline at bottom of request
        
    print(day)



if __name__ == "__main__":
    main(sys.argv[1])
