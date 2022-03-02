import sys
import json
ipinfo = json.loads(sys.stdin.read())
items = []
for k in ipinfo:
    items.append({
        "title": "%s:%s"%(k,ipinfo[k]),
        # "argv":ipinfo[k],
    })
print(json.dumps({"items": items}))