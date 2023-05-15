import json
import requests as r

data = r'{"version": "0.7.2", "package": {"name": "mindsdb", "ecosystem": "PyPI"}}'
res = r.post(r'https://api.osv.dev/v1/query', data=data)
print(res.text)