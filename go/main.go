package main

import (
	"bytes"
	"fmt"
	"net/http"

	spinhttp "github.com/fermyon/spin/sdk/go/v2/http"
	"github.com/fermyon/spin/sdk/go/v2/variables"
)

func init() {
	spinhttp.Handle(func(w http.ResponseWriter, r *http.Request) {
		token, err := variables.Get("token")
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		apiUri, err := variables.Get("api_uri")
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		version, err := variables.Get("version")
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		versionedApiUri := fmt.Sprintf("%s/%s", apiUri, version)

		request, err := http.NewRequest("GET", versionedApiUri, bytes.NewBuffer([]byte("")))
		request.Header.Add("Authorization", "Bearer "+token)
		response, err := spinhttp.Send(request)
		// fmt.Printf("Response: %v\n", response)
		// Do something with the response ...
		w.Header().Set("Content-Type", "text/plain")
		fmt.Fprintln(w, "Used an API")
	})
}

func main() {}
