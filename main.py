# we want to make a request to a website and parse the HTML content
import requests
from bs4 import BeautifulSoup, ResultSet
from threading import Thread
import json
from time import sleep

# make a request to the website
base_url = "https://tailwindcss.com"
response = requests.get('https://tailwindcss.com/docs/aspect-ratio')

soup = BeautifulSoup(response.text, 'html.parser')
tags_ = soup.find('h5', string='Backgrounds').parent.find('ul').find_all('a')

classes = {}
arbitrary_classes = {}

def get_info(url):
    req = requests.get(url)
    soup = BeautifulSoup(req.text, 'html.parser')
    # find for a href that contains #arbitrary-values if it does then we just add the class to the arbitrary_classes dictionary
    search = soup.find_all('tr')
    search_arb = soup.find_all('a', href='#arbitrary-values')
    for i in search:
        td = i.find_all('td')
        if len(td) >= 2:
            if len(search_arb) > 0:
                # if there is no nubmer in the end we just grab the first split for example text-2xl we will grab text but text-red-50 we will grab text-red
                # grab whatever is before the number for example bg-opacity-50 we will grab bg-opacity
                split_class = td[0].text.split("-")
                class_ = split_class[0] if not split_class[-1].isdigit() else "-".join(split_class[:-1])
                if class_ not in arbitrary_classes:
                    # replace all whatever is between : and ; with a placeholder like xxx
                    value = td[1].text
                    placeholder = td[1].text.replace(td[1].text[td[1].text.find(":") + 1:td[1].text.find(";")], " xxx")
                    # check if the value is a color or size
                    if "#" in value or "rgb" in value or "color:" in value:
                        arbitrary_classes[class_] = {"size": "", "color": placeholder, "position": ""}
                    elif "px" in value or "em" in value or "rem" in value or "%" in value:
                        arbitrary_classes[class_] = {"size": placeholder, "color": "", "position": ""}
                    else:
                        arbitrary_classes[class_] = {"size": "", "color": "", "position": placeholder}
                else:
                    value = td[1].text
                    placeholder = td[1].text.replace(td[1].text[td[1].text.find(":") + 1:td[1].text.find(";")], " xxx")
                    if "#" in value or "rgb" in value or "color:" in value:
                        arbitrary_classes[class_]["color"] = placeholder
                    elif "px" in value or "em" in value or "rem" in value or "%" in value:
                        arbitrary_classes[class_]["size"] = placeholder
                    else:
                        arbitrary_classes[class_]["position"] = placeholder
                
            classes[td[0].text] = td[1].text

threads = []

for i in tags_:
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
# print(json.dumps(classes, indent=4))
# write to a file
with open('data/backgrounds.json', 'w') as f:
    json.dump(classes, f, indent=4)

# print formatted arbitrary classes
# print(json.dumps(arbitrary_classes, indent=4))
# write to a file
with open('data/arbitrary_backgrounds.json', 'w') as f:
    json.dump(arbitrary_classes, f, indent=4)