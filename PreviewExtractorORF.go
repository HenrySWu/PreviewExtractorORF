// Extract the Preview JPEGs contained inside .ORF RAW files
// Place copy of exiftools next to script
// Place RAWs inside of RAW folder
// After running script, JPEGs will appear in JPEG folder

package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"sync"
)

func exiftoolcall(command1, command2 string) {
	exec.Command("cmd", "/C", command1).Output()
	msg, _ := exec.Command("cmd", "/C", command2).Output()
	fmt.Printf("%s", msg)
}

func main() {
	directory, err := os.Open("./")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	os.Mkdir("JPEG", 0777)
	err = os.Mkdir("RAW", 0777)
	if err == nil {
		fmt.Println("No RAW folder found, one has been made")
		os.Exit(1)
	}
	directory.Close()

	directory, err = os.Open("./RAW")
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer directory.Close() // close this directory when scope ends

	files, err := directory.ReadDir(-1) // used to verify that what os.Open has opened is a directory
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	var wg sync.WaitGroup // create coroutine group
	for _, file := range files {
		if filepath.Ext(file.Name()) == ".ORF" {
			fmt.Println("Extracting JPEG from", file.Name())
			cmdstring1 := "exiftool.exe -b -PreviewImage RAW/" + file.Name()
			cmdstring1 += " > JPEG/" + file.Name()[:len(file.Name())-3] + "jpg"
			cmdstring2 := "exiftool.exe -overwrite_original -TagsFromFile RAW/" + file.Name()
			cmdstring2 += " -exif:all JPEG/" + file.Name()[:len(file.Name())-3] + "jpg"
			wg.Add(1) // increment wait group counter by 1, used by Wait()
			go func() {
				exiftoolcall(cmdstring1, cmdstring2)
				wg.Done() // decrements wait group counter by 1
			}()
		}
	}
	wg.Wait() // holds until wait group counter hits 0, signalling that all coroutines have finished
	fmt.Println("\nFinished")
}
