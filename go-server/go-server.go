package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
	"strconv"
	"strings"
)

var count = 0

func handleConnection(conn net.Conn) {
	fmt.Print(".")

	for {
		message, err := bufio.NewReader(conn).ReadString(('\n'))
		if err != nil {
			fmt.Println(err)
			return
		}

		tmp := strings.TrimSpace(string(message))
		if tmp == "STOP" {
			break
		}

		fmt.Println(tmp)
		counter := strconv.Itoa(count) + "\n"

		// Reverse the message and send it back
		conn.Write([]byte(string(counter)))
	}

	conn.Close()
}

func main() {
	arguments := os.Args
	if len(arguments) == 1 {
		fmt.Println("Argument Error: Please provide a port")
		return
	}

	PORT := ":" + arguments[1]
	listen, err := net.Listen("tcp4", PORT)
	if err != nil {
		fmt.Println(err)
		return
	}

	defer listen.Close()

	for {
		conn, err := listen.Accept()
		if err != nil {
			fmt.Println(err)
			return
		}

		go handleConnection(conn)
		count++
	}
}
