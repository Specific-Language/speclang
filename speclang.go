package speclang

import (
	"fmt"
	"log"

	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hclwrite"
)

func Compile(input []byte) interface{} {
	result := parseHCL(input)

	fmt.Println(result)

	return result
}

func parseHCL(input []byte) *hclwrite.Body {
	origin := hcl.Pos{Byte: 0, Line: 1, Column: 1}
	hclInput, hclErr := hclwrite.ParseConfig(input, "input", origin)
	if hclErr != nil {
		log.Println("Failed to parse input as HCL")
		log.Fatalln(hclErr)
	}
	return hclInput.Body()
}
