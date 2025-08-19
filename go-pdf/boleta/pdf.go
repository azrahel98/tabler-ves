package boleta

import (
	"fmt"
	"gopdf/models"

	"github.com/johnfercher/maroto/v2"
	"github.com/johnfercher/maroto/v2/pkg/components/text"
	"github.com/johnfercher/maroto/v2/pkg/config"
	"github.com/johnfercher/maroto/v2/pkg/consts/align"
	"github.com/johnfercher/maroto/v2/pkg/consts/border"
	"github.com/johnfercher/maroto/v2/pkg/consts/fontstyle"
	"github.com/johnfercher/maroto/v2/pkg/consts/pagesize"
	"github.com/johnfercher/maroto/v2/pkg/core"
	"github.com/johnfercher/maroto/v2/pkg/props"
	"github.com/johnfercher/maroto/v2/pkg/repository"
)

func MakePdf(name string, persona models.Boleta) {

	m := GetMaroto(persona)

	d, err := m.Generate()
	if err != nil {
		fmt.Println(err)
	}
	err = d.Save(fmt.Sprintf("%s/%s.pdf", "./", name))
	if err != nil {
		fmt.Println(err)
	}

}

func GetMaroto(persona models.Boleta) core.Maroto {

	customFont := "Ubuntu"

	customFonts, err := repository.New().
		AddUTF8Font(customFont, fontstyle.Normal, "./asset/fonts/Ubuntu-Regular.ttf").
		AddUTF8Font(customFont, fontstyle.Bold, "./asset/fonts/Ubuntu-Bold.ttf").
		Load()
	if err != nil {
		fmt.Println(err)
	}

	cfg := config.NewBuilder().
		WithPageSize(pagesize.A4).
		WithLeftMargin(8).
		WithRightMargin(8).
		WithTopMargin(5).
		WithAuthor("Raul Chercca MVES", true).
		WithCustomFonts(customFonts).
		Build()

	mrt := maroto.New(cfg)

	m := maroto.NewMetricsDecorator(mrt)

	getTopBoleta(m)

	m.AddRow(6)
	m.AddRow(7, text.NewCol(12, "Datos del Trabajador", props.Text{
		Size:   9,
		Top:    1.5,
		Style:  fontstyle.Bold,
		Align:  align.Center,
		Left:   2.4,
		Family: "Ubuntu",
	}),
	).WithStyle(&props.Cell{
		BorderColor: getGrayColor(),
		BorderType:  border.Full, BorderThickness: 0.5,
	})
	getInforTrabajador(m, persona)
	m.AddRow(3)
	getDiasLaborados(m, persona)

	m.AddRow(4)

	getCuadroIngresos(m, persona)
	m.AddRow(3)

	m.AddRow(5, text.NewCol(12, "INGRESOS", props.Text{
		Size:  8,
		Style: fontstyle.Bold,
	}))

	for _, x := range persona.Ingresos {
		m.AddRow(4.5, text.NewCol(3, x.Codigo, props.Text{
			Align:  align.Left,
			Size:   9,
			Family: "Ubuntu",
		}), text.NewCol(7, x.Nombre, props.Text{
			Align:  align.Left,
			Size:   9,
			Family: "Ubuntu",
		}), text.NewCol(2, x.Monto, props.Text{
			Align:  align.Left,
			Size:   8,
			Family: "Ubuntu",
		}))
	}

	m.AddRow(2)
	m.AddRow(8, text.NewCol(8, ""),
		text.NewCol(2, "TOTAL INGRESOS", props.Text{
			Size:  8,
			Style: fontstyle.Bold,
		}),
		text.NewCol(2, "1235.56", props.Text{
			Size: 9,
		})).WithStyle(&props.Cell{BorderThickness: .4, BorderColor: getGrayColor(), BorderType: border.Bottom})

	m.AddRow(8, text.NewCol(12, "DECUENTOS",
		props.Text{
			Align:  align.Left,
			Bottom: 3,
			Top:    2,
			Size:   8,
			Style:  fontstyle.Bold,
		}))

	for _, x := range persona.Descuentos {
		m.AddRow(4.5, text.NewCol(3, x.Codigo, props.Text{
			Align:  align.Left,
			Size:   9,
			Family: "Ubuntu",
		}), text.NewCol(7, x.Nombre, props.Text{
			Align:  align.Left,
			Size:   9,
			Family: "Ubuntu",
		}), text.NewCol(2, x.Monto, props.Text{
			Align:  align.Left,
			Size:   8,
			Family: "Ubuntu",
		}))
	}

	m.AddRow(2)
	m.AddRow(8, text.NewCol(6, ""),
		text.NewCol(4, "TOTAL DESCUENTOS", props.Text{
			Size:  8,
			Style: fontstyle.Bold,
			Right: 6,
			Align: align.Right,
		}),
		text.NewCol(2, "1235.56", props.Text{
			Size: 9,
		})).WithStyle(&props.Cell{BorderThickness: .4, BorderColor: getGrayColor(), BorderType: border.Bottom})

	m.AddRow(8, text.NewCol(12, "APORTES",
		props.Text{
			Align:  align.Left,
			Bottom: 3,
			Top:    2,
			Size:   8,
			Style:  fontstyle.Bold,
		}))

	for _, x := range persona.AportesTrabajador {
		m.AddRow(4.5, text.NewCol(3, x.Codigo, props.Text{
			Align:  align.Left,
			Size:   9,
			Family: "Ubuntu",
		}), text.NewCol(7, x.Nombre, props.Text{
			Align:  align.Left,
			Size:   9,
			Family: "Ubuntu",
		}), text.NewCol(2, x.Monto, props.Text{
			Align:  align.Left,
			Size:   8,
			Family: "Ubuntu",
		}))
	}

	m.AddRow(8, text.NewCol(8, ""),
		text.NewCol(2, "TOTAL APORTES", props.Text{
			Size:  8,
			Style: fontstyle.Bold,
		}),
		text.NewCol(2, "1235.56", props.Text{
			Size: 9,
		}))

	m.AddRow(5)
	m.AddRow(5, text.NewCol(6, ""), text.NewCol(4, "NETO A PAGAR", props.Text{Style: fontstyle.Bold}),
		text.NewCol(2, persona.Neto, props.Text{Align: align.Left, Style: fontstyle.Bold}))

	return m
}

func getGrayColor() *props.Color {
	return &props.Color{
		Red:   200,
		Green: 200,
		Blue:  200,
	}
}

func getHeaderBackground() *props.Color {
	return &props.Color{
		Red:   48,
		Green: 62,
		Blue:  82,
	}
}
