# This file will host various functions that connect to
# weheartit.com's actual API endpoints.

# --------------------
import requests
from json import loads
from weheartpy.models import User, Entry
import user_agent
# --------------------


class Api():
    """
    This is the backbone of the WeHeartIt class
    Only observe it if you're sure you know whats happening down here!

    """
    def __init__(self) -> None:
        ua = user_agent.generate_user_agent()
        headers = {'User-agent': ua}
        err = requests.get("https://weheartit.com", headers=headers)
        try:
            err.raise_for_status()
        except requests.HTTPError:
                raise ConnectionError(f"The API cannot connect to weheartit.com\nREASON: {err.reason}\nhelp: Try checking your network connection.")
        self.status = err.status_code
        self.url = err.url

    def entry(self, entryid: int) -> dict:
        end = f"https://weheartit.com/api/v2/search/entries/{entryid}"
        end = (requests.get(end)).text
        res = loads(end)

        entrydata = {}
        entrydata["id"] = res["id"]
        entrydata["url"] = res["url"]
        entrydata["image"] = res["media"][0]["url"]
        entrydata["title"] = res["title"]
        entrydata["type"] = res["media_type"]
        entrydata["hearts"] = res["hearts_count"]
        entrydata["views"] = res["views_count"]
        entrydata["interactions"] = res["interactions_count"]
        entrydata["creation"] = res["created_at"]
        entrydata["via"] = res["via"]
        entrydata["tags"] = [tag["name"] for tag in res["tags"]]


        userdata = {}
        userdata["id"] = res["creator"]["id"]
        userdata["username"] = res["creator"]["username"]
        userdata["name"] = res["creator"]["name"]
        userdata["avatar"] = res["creator"]["avatar"][1]["url"]
        userdata["public"] = res["creator"]["public_account"]
        userdata["verified"] = res["creator"]["verified"]
        userdata["badges"] = res["creator"]["badges"]

        entrydata["user"] = User(userdata)

        entry = Entry(entrydata)
        return entry




        
        