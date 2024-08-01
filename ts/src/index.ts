import { HandleRequest, HttpRequest, HttpResponse, Config } from "@fermyon/spin-sdk"

export const handleRequest: HandleRequest = async function (request: HttpRequest): Promise<HttpResponse> {
  let token = Config.get("token")
  let apiUri = Config.get("api_uri")
  let version = Config.get("version")
  let versionedAPIUri = `${apiUri}/${version}`
  let response = await fetch(
    versionedAPIUri,
    {
      headers: {
        'Authorization': 'Bearer ' + token
      }
    }
  );
  // Do something with the response ...
  return {
    status: 200,
    headers: { "content-type": "text/plain" },
    body: "Used an API"
  }
}
