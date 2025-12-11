package main

import (
	"encoding/json"
	"fmt"
	"log"
	"strings"

	"github.com/ledongthuc/pdf"
)

type JSONResponse struct {
	Person string `json:"person"`
	Date   string `json:"date"`
	Hour   string `json:"hour"`
}

func extractValue(text, key string) string {
	idx := strings.Index(text, key)
	if idx == -1 {
		return ""
	}

	rest := text[idx+len(key):]
	lines := strings.Split(rest, "\n")

	for _, ln := range lines {
		ln = strings.TrimSpace(ln)
		if ln != "" {
			return ln
		}
	}

	return ""
}

func main() {
	file, reader, err := pdf.Open("../ponto.pdf")
	if err != nil {
		log.Fatalf("Erro abrindo PDF: %v", err)
	}
	defer file.Close()

	var out strings.Builder

	totalPages := reader.NumPage()
	for i := 1; i <= totalPages; i++ {
		p := reader.Page(i)
		if p.V.IsNull() {
			continue
		}
		content, _ := p.GetPlainText(nil)
		out.WriteString(content)
		out.WriteString("\n--- Fim da página ---\n")
	}

	text := out.String()

	person := extractValue(text, "PESSOA:")
	mark := extractValue(text, "MARCAÇÃO:")

	parts := strings.Fields(mark)

	date := ""
	hour := ""

	if len(parts) > 0 {
		date = parts[0]
	}
	if len(parts) > 1 {
		hour = parts[1]
	}

	res := JSONResponse{
		Person: person,
		Date:   date,
		Hour:   hour,
	}

	jsonBytes, _ := json.MarshalIndent(res, "", "  ")

	fmt.Println(string(jsonBytes))
}
