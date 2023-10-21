import json, sys

matches = json.load(open("content.json", "r"))
new_matches = [
    { m["players"][i] : m["scores"][i] for i in range(len(m["players"])) }
    for m in matches
]
with open('new-content.json', 'w', encoding='utf-8') as f:
    json.dump(new_matches, f, ensure_ascii=False, indent=4)
