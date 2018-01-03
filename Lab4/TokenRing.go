package main

import (
	"fmt"
)

type Token struct{
	data string
	recipient, ttl int
}

type TokenRingNode struct{
	i int
	in chan Token
	out chan Token
}

func (t TokenRingNode) Run(){
	for {
		token := <-t.in
		if token.recipient == t.i {
			fmt.Printf("Node (%d): Token %s received! \n", t.i, token.data)
		} else {
			if (token.ttl > 0) {
				token.ttl = token.ttl - 1
				t.out <- token
			} else{
				fmt.Printf("Node (%d): Time is out! \n", t.i)
			}
		}
	}
}

func main() {
	fmt.Printf("Enter number of nodes: ")
	var n int
	fmt.Scanf("%d", &n)

	var nodes []TokenRingNode
	nodes = append(nodes, TokenRingNode{0, nil, make(chan Token)})
	for i := 1; i < n; i++ {
		nodes = append(nodes, TokenRingNode{i, nodes[i-1].out, make(chan Token)})
	}
	nodes[0].in = nodes[n-1].out

	for i := 0; i < n; i++ {
		go nodes[i].Run()
	}

	fmt.Printf("Enter new message (recipient, time): ")
	var data string = "=^.^="
	var recipient, time int
	fmt.Scanf("%d %d", &recipient, &time)
	nodes[0].in <- Token{data, recipient, time}

	fmt.Scanln()
}