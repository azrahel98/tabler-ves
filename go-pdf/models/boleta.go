package models

type Boleta struct {
	Ruc               string
	Empleador         string
	Fecha             string
	TipoDocIdentidad  string
	NumeroDoc         string
	Nombre            string
	Situacion         string
	Ingreso           string
	TipoTrabajador    string  //regimen
	RegPensionario    string  // onp o afp
	Cussp             *string // si es onp no tiene
	DiasLab           string
	DiasnoLab         string // faltas
	DiasSubs          string // descansomed
	Condicion         string // domiciliado
	Holaslab          string
	TipoSuspe         string
	MoitvoSuspe       string
	DiasSuspe         string
	Ingresos          []Item
	Descuentos        []Item
	AportesTrabajador []Item
	Neto              string
	AporteEmpleador   []Item
}

type Item struct {
	Codigo string
	Nombre string
	Monto  string
}

type Table struct {
	Rows []Row `xml:"Row"`
}

type Row struct {
	Cells []Cell `xml:"Cell"`
}

type Cell struct {
	Data string `xml:"Data"`
}

type ListOfFileDirectory struct {
	Dni  string
	Mes  string
	Año  string
	Full string
}
