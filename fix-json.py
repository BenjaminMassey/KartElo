import json, sys

matches = json.load(open("content.json", "r"))
new_matches = [
    { m["players"][i] : m["scores"][i] for i in range(len(m["players"])) }
    for m in matches
]
json.dump(new_matches, sys.stdout)
