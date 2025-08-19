package boleta

import (
	"gopdf/models"

	"github.com/johnfercher/maroto/v2/pkg/components/image"
	"github.com/johnfercher/maroto/v2/pkg/components/text"
	"github.com/johnfercher/maroto/v2/pkg/consts/align"
	"github.com/johnfercher/maroto/v2/pkg/consts/border"
	"github.com/johnfercher/maroto/v2/pkg/consts/fontstyle"
	"github.com/johnfercher/maroto/v2/pkg/core"
	"github.com/johnfercher/maroto/v2/pkg/props"
)

func getTopBoleta(m core.Maroto) []core.Row {
	return []core.Row{
		m.AddRow(18, image.NewFromFileCol(1, "./asset/logo.png", props.Rect{
			Percent: 100,

			JustReferenceWidth: true,
		}),

			text.NewCol(4, "Municipalidad Distrital", props.Text{
				Align:  align.Left,
				Size:   12,
				Family: "Ubuntu",
				Style:  fontstyle.Bold,
				Top:    3,
			}),
			text.NewCol(-4, "de Villa el Salvador", props.Text{
				Align:  align.Left,
				Size:   10,
				Top:    7.5,
				Family: "Ubuntu",
			}),
			text.NewCol(4, "Lima - Peru", props.Text{
				Align:  align.Left,
				Size:   10,
				Top:    11,
				Family: "Ubuntu",
			}),
			text.NewCol(5, ""),
			text.NewCol(2, "N° 000015564", props.Text{
				Top:   3,
				Align: align.Right,
				Style: fontstyle.Bold,
				Size:  13,
			}),
			text.NewCol(-2, "01/01/2025", props.Text{
				Top:   8,
				Align: align.Right,
			}),
			text.NewCol(2, "10:00:00", props.Text{
				Top:   11.5,
				Align: align.Right,
			}),
		),
	}
}

func getInforTrabajador(m core.Maroto, persona models.Boleta) []core.Row {
	return []core.Row{
		m.AddAutoRow(text.NewCol(2, "Doc. de Identidad :", props.Text{
			Align:  align.Left,
			Size:   9,
			Family: "Ubuntu",
		}),
			text.NewCol(2, persona.NumeroDoc, props.Text{
				Align:  align.Left,
				Size:   9,
				Style:  fontstyle.Bold,
				Family: "Ubuntu",
			}),
			text.NewCol(2, "Codigo AIRHSP:", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
			text.NewCol(2, persona.NumeroDoc, props.Text{
				Align:  align.Left,
				Size:   9,
				Style:  fontstyle.Bold,
				Family: "Ubuntu",
			}),
			text.NewCol(2, "Fecha de Ingreso:", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
			text.NewCol(2, persona.Ingreso, props.Text{
				Align:  align.Left,
				Size:   9,
				Style:  fontstyle.Bold,
				Family: "Ubuntu",
			}),
		),
		m.AddRow(.8),

		m.AddAutoRow(text.NewCol(2, "Apellidos y Nombres:", props.Text{
			Align:  align.Left,
			Size:   9,
			Family: "Ubuntu",
		}),
			text.NewCol(4, "Raul Smith Taylor Choque de Quispe", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
			text.NewCol(2, "Regimen Laboral:", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
			text.NewCol(4, "Ley N° 30057 - Ley SERVIR", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
		),
		m.AddRow(.8),

		m.AddAutoRow(text.NewCol(1, "Area :", props.Text{
			Align:  align.Justify,
			Size:   9,
			Family: "Ubuntu",
		}),
			text.NewCol(5, "SUBGERENCIA DE SERENAZGO", props.Text{
				Align:  align.Left,
				Family: "Ubuntu",
				Size:   9,
			}),
			text.NewCol(1, "Cargo:", props.Text{
				Align:  align.Left,
				Family: "Ubuntu",
				Size:   9,
			}),
			text.NewCol(5, "ABOGADA", props.Text{
				Align:  align.Left,
				Style:  fontstyle.Bold,
				Size:   8,
				Family: "Ubuntu",
			}),
		),
		m.AddRow(.8),
		m.AddAutoRow(text.NewCol(2, "Regimen Pensionario:", props.Text{
			Align:  align.Left,
			Size:   9,
			Family: "Ubuntu",
		}),
			text.NewCol(2, persona.RegPensionario, props.Text{
				Align:  align.Left,
				Size:   8,
				Family: "Ubuntu",
			}),
			text.NewCol(2, "Administrador", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
			text.NewCol(2, persona.RegPensionario, props.Text{
				Align:  align.Left,
				Family: "Ubuntu",
				Size:   9,
			}),
			text.NewCol(2, "CUSPP", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
			text.NewCol(2, *persona.Cussp, props.Text{
				Align:  align.Left,
				Family: "Ubuntu",
				Size:   9,
			}),
		),
		m.AddRow(.8),
		m.AddRow(6,

			text.NewCol(2, "Grupo Ocupacional:", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
			text.NewCol(2, "NO APLICA", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
			text.NewCol(2, "Cargo Estructural:", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
			text.NewCol(2, "NO APLICA", props.Text{
				Align:  align.Left,
				Family: "Ubuntu",
				Size:   9,
			}),
			text.NewCol(2, "Jornada Laboral:", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
			text.NewCol(2, "48 Horas", props.Text{
				Align:  align.Left,
				Family: "Ubuntu",
				Size:   9,
			}),
		),
	}
}

func getDiasLaborados(m core.Maroto, persona models.Boleta) []core.Row {
	return []core.Row{
		m.AddRow(6, text.NewCol(3, "Dias Laborados:	30", props.Text{
			Align:  align.Left,
			Size:   9,
			Family: "Ubuntu",
		}),
			text.NewCol(3, "Dias No Laborados:	4", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
			text.NewCol(3, "Dias Subsidiados:	4", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
			text.NewCol(3, "Periodos Vacacional:	3", props.Text{
				Align:  align.Left,
				Size:   9,
				Family: "Ubuntu",
			}),
		),
	}
}

func getCuadroIngresos(m core.Maroto, persona models.Boleta) []core.Row {
	return []core.Row{

		m.AddRow(7, text.NewCol(3, "CODIGO", props.Text{
			Size:   9,
			Top:    1.5,
			Style:  fontstyle.Bold,
			Left:   2.4,
			Family: "Ubuntu",
		}),
			text.NewCol(7, "CONCEPTO", props.Text{
				Size:   9,
				Top:    1.5,
				Style:  fontstyle.Bold,
				Align:  align.Left,
				Family: "Ubuntu",
			}), text.NewCol(2, "MONTO", props.Text{
				Size:   9,
				Top:    1.5,
				Style:  fontstyle.Bold,
				Align:  align.Left,
				Family: "Ubuntu",
			})).WithStyle(&props.Cell{
			BorderColor: getGrayColor(),
			BorderType:  border.Full, BorderThickness: 0.5,
		}),
	}
}
