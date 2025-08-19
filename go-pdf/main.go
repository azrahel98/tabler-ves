package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/chromedp/cdproto/page"
	"github.com/chromedp/chromedp"
)

func main() {
	html := "./index.html"
	chromeExe := `C:/Users/olaf/Downloads/chrlauncher-win64-stable-codecs-sync/bin/chrome.exe`

	for i := range 5 {
		pdfOut := fmt.Sprintf("./pdf/boleta%d.pdf", i)

		content, err := os.ReadFile(html)
		if err != nil {
			log.Fatalf("Error al leer el archivo: %v", err)
		}

		htmlString := string(content)
		fmt.Printf(fmt.Sprintf("0%d/2025", i))
		htmlString = strings.ReplaceAll(htmlString, "@@PERIODO@@", fmt.Sprintf("0%d/2025", i+1))
		htmlString = strings.ReplaceAll(htmlString, "@@DNI@@", "70247264")
		htmlString = strings.ReplaceAll(htmlString, "@@NOMBRE@@", "RAUL SMITH CHERCCA LOPEZ")
		htmlString = strings.ReplaceAll(htmlString, "@@CODIGOAIR@@", "000555")

		htmltem := "./pdf/temp.html"

		if err := os.WriteFile(htmltem, []byte(htmlString), 0644); err != nil {
			log.Fatalf("Error guardando archivo temporal: %v", err)
		}

		abs, err := filepath.Abs(htmltem)
		if err != nil {
			panic(err)
		}
		url := "file:///" + filepath.ToSlash(abs)

		allocCtx, cancel := chromedp.NewExecAllocator(context.Background(),
			append(chromedp.DefaultExecAllocatorOptions[:],
				chromedp.ExecPath(chromeExe),
				chromedp.NoFirstRun,
				chromedp.NoDefaultBrowserCheck,
				chromedp.Flag("headless", true),
				chromedp.Flag("disable-gpu", true),
				chromedp.Flag("allow-file-access-from-files", true),
				chromedp.Flag("disable-extensions", true),
			)...,
		)
		defer cancel()

		ctx, cancel2 := chromedp.NewContext(allocCtx)
		defer cancel2()

		ctx, cancel3 := context.WithTimeout(ctx, 30*time.Second)
		defer cancel3()

		var pdf []byte
		err = chromedp.Run(ctx,
			chromedp.Navigate(url),
			chromedp.WaitReady("body", chromedp.ByQuery),
			chromedp.ActionFunc(func(ctx context.Context) error {
				buf, _, err := page.PrintToPDF().
					WithPrintBackground(true).
					WithDisplayHeaderFooter(false).
					WithMarginTop(0).
					WithMarginBottom(0).
					WithMarginLeft(0).
					WithMarginRight(0).
					WithGenerateTaggedPDF(false).
					WithPreferCSSPageSize(true).
					WithScale(1).
					Do(ctx)
				if err != nil {
					return err
				}
				pdf = buf
				return nil
			}),
		)
		if err != nil {
			fmt.Println("❌ Error generando PDF:", err)
			return
		}
		if err := os.WriteFile(pdfOut, pdf, 0644); err != nil {
			fmt.Println("❌ Error guardando PDF:", err)
			return
		}
		fmt.Println("✅ PDF generado en:", pdfOut)
		if err := os.Remove(htmltem); err != nil && !os.IsNotExist(err) {
			fmt.Println("❌ Error eliminando archivo:", err)
		}
	}
}
