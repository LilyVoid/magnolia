/// basic HTTP server, handing only a single path (/)

/// fr though, i want GO to handle the API stuff
/// Rust for the uhh other complicated things that could be slow on GO

/// uhh this'll be unfinished for now, because yeah

package main

import (
	"fmt"
	"encoding/json"
	"net/http"
	"io"
)

var (
	commandList map[Action]any
)

type request struct {
	Action  `json:"action"`
	RequestData map[string]any `json:"requestData"`
}

type response struct {
	YourCommand string `json:"yourCommand"`
	ResponseData map[string]any `json:"responseData"`
}

type Action struct {
	description string
	execute func
}

/// list of available actions i guess

func (act *Action) echo() {
	/// basic echo action
	body, err := io.ReadAll(req.Body)
	if err != nil {
		fmt.Println(err) /// continue main function execution though
		http.Error(res, "Your JSON was not processed properly", http.StatusBadRequest)
		return
	}
	defer req.Body.Close()

	w.Write(body)
}

func init() {

}

func handler(res http.ResponseWriter, req *http.Request) {

}

func main() {
	http.HandleFunc("/", )
}
