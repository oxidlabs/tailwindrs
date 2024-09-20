# we want to make a request to a website and parse the HTML content
import requests
from bs4 import BeautifulSoup, ResultSet
from threading import Thread
import json

# make a request to the website
base_url = "https://tailwindcss.com"
response = requests.get('https://tailwindcss.com/docs/aspect-ratio')

# we want to look for a li element with a h5 element inside with the content being Layout and we want to get all the a tags that is nested inside the ul tag which is a child of
# the li tag
# parse the HTML content
soup = BeautifulSoup(response.text, 'html.parser')
search = soup.find('h5', string='Layout').parent.find('ul').find_all('a')

classes = {}
arbitrary_classes = set()

def get_info(url):
    req = requests.get(url)
    soup = BeautifulSoup(req.text, 'html.parser')
    # find for a href that contains #arbitrary-values if it does then we just add the class to the arbitrary_classes dictionary
    search = soup.find_all('tr')
    search_arb = soup.find_all('a', href='#arbitrary-values')
    for i in search:
        td = i.find_all('td')
        if len(td) == 2:
            if len(search_arb) > 0:
                arbitrary_classes.add(td[0].text.split("-")[0])
                arb_val = True
            # remove the \n from td[1].text
            classes[td[0].text] = td[1].text

threads = []

for i in search:
    # we will loop through each href and make a request to get more information
    url = base_url + i['href']
    thread = Thread(target=get_info, args=(url,))
    thread.daemon = True
    thread.start()
    threads.append(thread)

# wait for all threads to finish
for thread in threads:
    thread.join()

# print formatted classes 
print(json.dumps(classes, indent=4))
print(arbitrary_classes)