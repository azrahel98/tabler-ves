<template>
  <div class="boleta-container" id="boleta">
    <div class="header">
      <div class="header-top">
        <div class="logo-section">
          <div class="logo">🏛️</div>
          <div class="entity-info">
            <h2>{{ entidad.nombre }}</h2>
            <p>{{ entidad.distrito }}</p>
          </div>
        </div>
        <div class="boleta-number">
          <h1>N° {{ boletaNumero }}</h1>
          <p>{{ fechaGeneracion }} - {{ horaGeneracion }}</p>
        </div>
      </div>
      <div class="period-badge">Período: {{ periodo }}</div>
    </div>

    <div class="content">
      <div class="employee-section">
        <div class="employee-grid">
          <div class="info-group" v-for="campo in camposEmpleado" :key="campo.key">
            <span class="info-label">{{ campo.label }}</span>
            <span class="info-value" :class="{ 'large-text': campo.largeText }">
              {{ campo.value }}
            </span>
          </div>
        </div>
      </div>

      <div class="financial-section">
        <!-- Ingresos -->
        <div class="card ingresos">
          <div class="card-header">
            <div class="card-icon">💰</div>
            <div class="card-title">INGRESOS</div>
          </div>
          <div class="item" v-for="ingreso in ingresos" :key="ingreso.codigo">
            <span class="item-name">{{ ingreso.concepto }}</span>
            <span class="item-amount">{{ formatearMonto(ingreso.monto) }}</span>
          </div>
          <div class="item">
            <span class="item-name">TOTAL INGRESOS</span>
            <span class="item-amount">{{ formatearMonto(totalIngresos) }}</span>
          </div>
        </div>

        <!-- Descuentos -->
        <div class="card descuentos">
          <div class="card-header">
            <div class="card-icon">📉</div>
            <div class="card-title">DESCUENTOS</div>
          </div>
          <div class="item" v-for="descuento in descuentos" :key="descuento.codigo">
            <span class="item-name">{{ descuento.concepto }}</span>
            <span class="item-amount">{{ formatearMonto(descuento.monto) }}</span>
          </div>
          <div class="item">
            <span class="item-name">TOTAL DESCUENTOS</span>
            <span class="item-amount">{{ formatearMonto(totalDescuentos) }}</span>
          </div>
        </div>

        <!-- Aportes -->
        <div class="card aportes">
          <div class="card-header">
            <div class="card-icon">🏥</div>
            <div class="card-title">APORTES</div>
          </div>
          <div class="item" v-for="aporte in aportes" :key="aporte.codigo">
            <span class="item-name">{{ aporte.concepto }}</span>
            <span class="item-amount">{{ formatearMonto(aporte.monto) }}</span>
          </div>
          <div class="item">
            <span class="item-name">TOTAL APORTES</span>
            <span class="item-amount">{{ formatearMonto(totalAportes) }}</span>
          </div>
        </div>
      </div>

      <div class="total-section">
        <div class="total-label">NETO A PAGAR</div>
        <div class="total-amount">{{ formatearMonto(netoAPagar) }}</div>
        <div class="total-currency">Soles Peruanos</div>
      </div>
    </div>

    <div class="footer">
      <p>Este documento es generado automáticamente por el sistema de planillas de {{ entidad.nombre }}</p>
      <p>Para consultas o reclamos, dirigirse al área de Recursos Humanos</p>
    </div>
  </div>

  <button @click="generarpdf">Generar</button>
</template>

<script>
export default {
  name: 'BoletaPago',
  props: {
    empleadoData: {
      type: Object,
      default: () => ({
        dni: '45678912',
        codigoAirhsp: '45678912',
        nombres: 'Raúl Smith Taylor Choque de Quispe',
        cargo: 'Especialista en Planeamiento',
        area: 'Unidad de Planeamiento Estratégico',
        fechaIngreso: '2020-01-15',
        cuspp: '123456789012',
        regimenLaboral: 'CAS',
        regimenPensionario: 'AFP Integra',
        rubroFinanciamiento: '09',
        metaPresupuestal: '11',
        condicion: 'DOMICILIADO',
        jornadaLaboral: '48 HORAS',
        diasTrabajados: 30,
        diasNoLaborados: 4,
        diasSubsidiados: 4,
        periodosVacacional: 3,
        establecimiento: 'Villa el Salvador'
      })
    },
    entidadData: {
      type: Object,
      default: () => ({
        nombre: 'Municipalidad Distrital',
        distrito: 'Villa el Salvador'
      })
    },
    boletaInfo: {
      type: Object,
      default: () => ({
        numero: '00000001',
        periodo: 'Mayo - 2025',
        fecha: '01/01/2025',
        hora: '10:01:01'
      })
    },
    conceptosData: {
      type: Object,
      default: () => ({
        ingresos: [
          { codigo: '001', concepto: 'Sueldo Básico', monto: 3000.0 },
          { codigo: '002', concepto: 'Asignación Familiar', monto: 102.5 },
          { codigo: '003', concepto: 'Bonificación Extraordinaria', monto: 200.0 }
        ],
        descuentos: [
          { codigo: '101', concepto: 'AFP Fondo', monto: 300.0 },
          { codigo: '102', concepto: 'AFP Comisión', monto: 45.0 },
          { codigo: '103', concepto: 'Renta Quinta Categoría', monto: 0.0 }
        ],
        aportes: [
          { codigo: '201', concepto: 'Essalud', monto: 270.0 },
          { codigo: '202', concepto: 'Seguro Vida Ley', monto: 15.0 }
        ]
      })
    }
  },
  computed: {
    entidad() {
      return this.entidadData
    },
    boletaNumero() {
      return this.boletaInfo.numero
    },
    fechaGeneracion() {
      return this.boletaInfo.fecha
    },
    horaGeneracion() {
      return this.boletaInfo.hora
    },
    periodo() {
      return this.boletaInfo.periodo
    },
    camposEmpleado() {
      return [
        { key: 'nombres', label: 'Apellidos y Nombres', value: this.empleadoData.nombres, largeText: true },
        { key: 'dni', label: 'DNI', value: this.empleadoData.dni },
        { key: 'codigoAirhsp', label: 'Código AIRHSP', value: this.empleadoData.codigoAirhsp },
        { key: 'cargo', label: 'Cargo', value: this.empleadoData.cargo, largeText: true },
        { key: 'area', label: 'Área', value: this.empleadoData.area, largeText: true },
        { key: 'fechaIngreso', label: 'Fecha de Ingreso', value: this.empleadoData.fechaIngreso },
        { key: 'cuspp', label: 'CUSPP', value: this.empleadoData.cuspp },
        { key: 'regimenLaboral', label: 'Régimen Laboral', value: this.empleadoData.regimenLaboral },
        { key: 'regimenPensionario', label: 'Régimen Pensionario', value: this.empleadoData.regimenPensionario },
        { key: 'rubroFinanciamiento', label: 'Rubro Financiamiento', value: this.empleadoData.rubroFinanciamiento },
        { key: 'metaPresupuestal', label: 'Meta Presupuestal', value: this.empleadoData.metaPresupuestal },
        { key: 'condicion', label: 'Condición', value: this.empleadoData.condicion },
        { key: 'jornadaLaboral', label: 'Jornada Laboral', value: this.empleadoData.jornadaLaboral },
        { key: 'diasTrabajados', label: 'Días Trabajados', value: this.empleadoData.diasTrabajados },
        { key: 'diasNoLaborados', label: 'Días No Laborados', value: this.empleadoData.diasNoLaborados },
        { key: 'diasSubsidiados', label: 'Días Subsidiados', value: this.empleadoData.diasSubsidiados },
        { key: 'periodosVacacional', label: 'Períodos Vacacional', value: this.empleadoData.periodosVacacional },
        { key: 'establecimiento', label: 'Establecimiento', value: this.empleadoData.establecimiento }
      ]
    },
    ingresos() {
      return this.conceptosData.ingresos
    },
    descuentos() {
      return this.conceptosData.descuentos
    },
    aportes() {
      return this.conceptosData.aportes
    },
    totalIngresos() {
      return this.ingresos.reduce((total, item) => total + item.monto, 0)
    },
    totalDescuentos() {
      return this.descuentos.reduce((total, item) => total + item.monto, 0)
    },
    totalAportes() {
      return this.aportes.reduce((total, item) => total + item.monto, 0)
    },
    netoAPagar() {
      return this.totalIngresos - this.totalDescuentos
    }
  },
  methods: {
    formatearMonto(monto) {
      return new Intl.NumberFormat('es-PE', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
      }).format(monto)
    },
    generarpdf() {
      const element = document.getElementById('boleta')
      const opt = {
        margin: 0,
        filename: 'myfile.pdf',
        image: { type: 'jpeg', quality: 0.98 },
        html2canvas: { scale: 2 },
        jsPDF: { unit: 'px', format: [element.offsetWidth, element.offsetHeight] } // tamaño dinámico
      }
      html2pdf().set(opt).from(element).save()
    }
  }
}
</script>

<style scoped>
#boleta {
  transform: scale(0.7);
  transform-origin: top left;
}
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.boleta-container {
  background: white;
  border-radius: 20px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  max-width: 800px;
  width: 100%;
  position: relative;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.header {
  background: linear-gradient(135deg, #1e293b 0%, #334155 50%, #475569 100%);
  color: white;
  padding: 30px;
  position: relative;
  overflow: hidden;
}

.header::before {
  content: '';
  position: absolute;
  top: -50%;
  right: -20%;
  width: 200px;
  height: 200px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 50%;
  animation: float 6s ease-in-out infinite;
}

@keyframes float {
  0%,
  100% {
    transform: translateY(0px) rotate(0deg);
  }
  50% {
    transform: translateY(-20px) rotate(10deg);
  }
}

.header-top {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 20px;
}

.logo-section {
  display: flex;
  align-items: center;
  gap: 15px;
}

.logo {
  width: 60px;
  height: 60px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 15px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  font-weight: bold;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.entity-info h2 {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 5px;
}

.entity-info p {
  font-size: 14px;
  opacity: 0.9;
}

.boleta-number {
  text-align: right;
  background: rgba(255, 255, 255, 0.08);
  padding: 15px;
  border-radius: 10px;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.boleta-number h1 {
  font-size: 24px;
  font-weight: 700;
  margin-bottom: 5px;
}

.boleta-number p {
  font-size: 12px;
  opacity: 0.8;
}

.period-badge {
  display: inline-block;
  background: rgba(255, 255, 255, 0.2);
  padding: 10px 20px;
  border-radius: 25px;
  font-size: 16px;
  font-weight: 600;
  backdrop-filter: blur(10px);
}

.content {
  padding: 30px;
}

.employee-section {
  background: linear-gradient(135deg, #f8f9ff 0%, #e8f2ff 100%);
  border-radius: 15px;
  padding: 25px;
  margin-bottom: 30px;
  border-left: 5px solid #475569;
}

.employee-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 15px 20px;
}

.info-group {
  display: flex;
  flex-direction: column;
}

.info-label {
  font-size: 11px;
  font-weight: 600;
  color: #666;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 4px;
}

.info-value {
  font-size: 14px;
  font-weight: 600;
  color: #333;
  background: white;
  padding: 6px 10px;
  border-radius: 6px;
  border: 1px solid #e0e6ed;
  min-height: 32px;
  display: flex;
  align-items: center;
}

.info-value.large-text {
  font-size: 12px;
  line-height: 1.2;
}

.financial-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 25px;
  margin-bottom: 30px;
}

.card {
  background: white;
  border-radius: 15px;
  padding: 20px;
  box-shadow: 0 5px 20px rgba(0, 0, 0, 0.08);
  border: 1px solid #f0f0f0;
  transition: all 0.3s ease;
}

.card:hover {
  transform: translateY(-5px);
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
}

.card-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 20px;
  padding-bottom: 15px;
  border-bottom: 2px solid #f5f5f5;
}

.card-icon {
  width: 35px;
  height: 35px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  font-weight: bold;
}

.ingresos .card-icon {
  background: linear-gradient(135deg, #4ade80, #22c55e);
  color: white;
}

.descuentos .card-icon {
  background: linear-gradient(135deg, #f87171, #ef4444);
  color: white;
}

.aportes .card-icon {
  background: linear-gradient(135deg, #60a5fa, #3b82f6);
  color: white;
}

.card-title {
  font-size: 18px;
  font-weight: 700;
  color: #333;
}

.item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  border-bottom: 1px solid #f0f0f0;
}

.item:last-child {
  border-bottom: none;
  font-weight: 600;
  padding-top: 15px;
  margin-top: 10px;
  border-top: 2px solid #e5e5e5;
}

.item-name {
  font-size: 14px;
  color: #555;
}

.item-amount {
  font-size: 14px;
  font-weight: 600;
  color: #333;
}

.total-section {
  background: linear-gradient(135deg, #1e293b, #334155);
  color: white;
  border-radius: 15px;
  padding: 30px;
  text-align: center;
  position: relative;
  overflow: hidden;
}

.total-section::before {
  content: '';
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: radial-gradient(circle, rgba(255, 255, 255, 0.1) 0%, transparent 70%);
  animation: pulse 4s ease-in-out infinite;
}

@keyframes pulse {
  0%,
  100% {
    opacity: 0.5;
  }
  50% {
    opacity: 1;
  }
}

.total-label {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 10px;
  position: relative;
  z-index: 1;
}

.total-amount {
  font-size: 36px;
  font-weight: 700;
  margin-bottom: 5px;
  position: relative;
  z-index: 1;
}

.total-currency {
  font-size: 14px;
  opacity: 0.8;
  position: relative;
  z-index: 1;
}

.footer {
  background: #f8fafc;
  padding: 20px 30px;
  border-top: 1px solid #e2e8f0;
  text-align: center;
  color: #64748b;
  font-size: 12px;
}

@media (max-width: 768px) {
  .employee-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 12px 15px;
  }

  .financial-section {
    grid-template-columns: 1fr;
  }

  .header-top {
    flex-direction: column;
    gap: 20px;
  }

  .boleta-number {
    text-align: left;
  }
}

@media (max-width: 480px) {
  .employee-grid {
    grid-template-columns: 1fr;
  }
}
</style>
