package main

import (
	"crypto/sha256"
	"flag"
	"fmt"
	"io"
	"os"
)

func HashFile(path string, chunksize int64) (string, int64) {
	hasher := sha256.New()

	file, err := os.Open(path)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

  var total int64 = 0
	var bRead int64 = -1
	for bRead != 0 {
		bRead, _ = io.CopyN(hasher, file, chunksize)
    total = total + bRead
	}

	bs := hasher.Sum(nil)
	return fmt.Sprintf("%x", bs), total
}

func main() {
  var path string
  flag.StringVar(&path, "path",  "/", "Path to backup")
  flag.Parse()

	h, b := HashFile(path, 4096)
  fmt.Printf("%s : %d\n", h, b)
}
