# Spin Application Variables API Examples

Contains a list of examples for each language for using Spin application variables. The examples are referenced from the [Spin application variables documentation](https://developer.fermyon.com/spin/v2/variables).


## Running the API Application

The (__very simple and insecure__) API will only return a response to applications that present the correct bearer token.

```sh
SPIN_VARIABLE_TOKEN=foo spin build --up --listen localhost:3001
```

A request with the appropriate token will get an "Authorized" response.

```sh
curl -H "Authorization: Bearer foo" localhost:3001
> Authorized
```


## Running the Language Applications

Run the application in the language of your choice, setting the expected token and API app URI as application variables using the Spin environment variable provider (using the `SPIN_VARIABLE` prefix):

```sh
SPIN_VARIABLE_TOKEN=foo SPIN_VARIABLE_API_URI="http://localhost:3001" spin build --up --listen localhost:3002
```

A request to the app should issue a successful request to the API.

```sh
curl localhost:3002
> Used an API
```
