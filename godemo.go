package main

import (
	"errors"
	"flag"
	"io"
	"log"
	"os"

	"gopkg.in/yaml.v3"
)

func mainE() error {
	f, err := os.Open(flag.Arg(0))
	if err != nil {
		return err
	}
	defer f.Close()

	dec := yaml.NewDecoder(f)
	enc := yaml.NewEncoder(os.Stdout)
	for {
		var i interface{}
		if err := dec.Decode(&i); errors.Is(err, io.EOF) {
			break
		} else if err != nil {
			return err
		}

		enc.Encode(i)
	}
	return nil
}

func main() {
	flag.Parse()
	if err := mainE(); err != nil {
		log.Fatal(err)
	}
}
