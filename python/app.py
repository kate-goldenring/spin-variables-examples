from spin_sdk.http import IncomingHandler, Request, Response, send
from spin_sdk import variables

class IncomingHandler(IncomingHandler):
    def handle_request(self, request: Request) -> Response:
        token = variables.get("token")
        api_uri = variables.get("api_uri")
        version = variables.get("version")
        versioned_api_uri = f"{api_uri}/{version}"
        headers = {
            "Authorization": f"Bearer {token}"
        }
        response = send(Request("GET", versioned_api_uri, headers, None))
        # Do something with the response ...
        return Response(
            200,
            {"content-type": "text/plain"},
            bytes("Used an API", "utf-8")
        )