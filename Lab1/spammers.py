import requests
import re
import time
queue = list()
list_emails = set()
unique_pages = set()

def make_url(href, current_url):
    if (re.search(r'^\w*:', href) != None):     #ссылка содержит http или https или другой протокол
        return href
    if (href.startswith("#")):                  #ссылки, начинающиеся на решетку - ссылки на другие части страницы
        return current_url + href
    if (href.startswith("/")):                  #ссылки от корня сайта
        return re.search(r'(https?://.*?)(?:/|$)',current_url).group(1) + href
    return re.search(r'(https?://.*)(?:/|$)',current_url).group(1) + "/" + href  #относительные ссылки

def clean_hash(full_href):
    if('#' in full_href):
        return re.search(r'^(.*?)#',full_href).group(1)
    else:
        return full_href

def process_url(siteurl, k):
    print (str(k + 1)+" loading "+siteurl)
    base_url=re.search(r'(https?://.*?)(?:/|$)',siteurl).group(1)
    response = requests.get(siteurl)
    content_type = response.headers.get('Content-Type')
    if not(content_type.startswith('text/html')):
        return

    emails = re.findall(r'[\w\d._-]+@(?:[\d\w\-]+\.)+[\w\d\-]+', response.text)
    for email in emails:
        list_emails.add(email)
    hrefs = re.findall(r"href=(?:\"([^\"]+)\"|\'([^\']+)\')", response.text)
    for href in hrefs:
        raw_href = (href[1] if href[0]=='' else href[0]) #есть группа, которая захватывает кавычки двойные и кавычки одинарные
        full_href = make_url(raw_href, siteurl)
        full_href = clean_hash(full_href)
        if (full_href.startswith(base_url) and (full_href not in unique_pages)):
            queue.append(full_href)
            unique_pages.add(full_href)

def run(decision):
    if (decision == 1):
        site_url = "http://www.csd.tsu.ru"
    else:
        site_url = "https://www.mosigra.ru"

    queue.append(site_url)
    unique_pages.add(site_url)
    k = 0
    while (len(queue) != 0 and k < 20):
        process_url(queue.pop(0), k)
        k += 1
        time.sleep(2)
    for email in list_emails:
        print(email)


decision = input("Choose the web-site (1 or 2): ")
while (decision != "1" and decision != "2"):
    decision = input("Choose again the web-site (1 or 2): ")
run(int(decision))