import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface Customer {
  id: string
  name: string
  email: string
  company: string
  status: 'Lead' | 'Contacted' | 'Proposal' | 'Won' | 'Lost'
  value: number
  createdAt: string
}

export const useCrmStore = defineStore('crm', () => {
  const customers = ref<Customer[]>([
    {
      id: '1',
      name: 'Carlos Mendoza',
      email: 'carlos.mendoza@innovatech.com',
      company: 'InnovaTech Solutions',
      status: 'Won',
      value: 12500,
      createdAt: '2026-08-15',
    },
    {
      id: '2',
      name: 'Valeria Ramos',
      email: 'v.ramos@logistica-andina.com',
      company: 'Logística Andina',
      status: 'Proposal',
      value: 8400,
      createdAt: '2026-08-20',
    },
    {
      id: '3',
      name: 'Diego Morales',
      email: 'diego@finanzasglobal.pe',
      company: 'Finanzas Global',
      status: 'Contacted',
      value: 15200,
      createdAt: '2026-08-24',
    },
    {
      id: '4',
      name: 'Lucía Fernández',
      email: 'lucia.f@retailnexus.com',
      company: 'Retail Nexus',
      status: 'Lead',
      value: 6300,
      createdAt: '2026-08-28',
    },
  ])

  const totalValue = computed(() =>
    customers.value.reduce((acc, curr) => acc + curr.value, 0)
  )

  const wonDealsCount = computed(
    () => customers.value.filter((c) => c.status === 'Won').length
  )

  function addCustomer(customer: Omit<Customer, 'id' | 'createdAt'>) {
    const newCustomer: Customer = {
      ...customer,
      id: Date.now().toString(),
      createdAt: new Date().toISOString().split('T')[0],
    }
    customers.value.unshift(newCustomer)
  }

  function deleteCustomer(id: string) {
    customers.value = customers.value.filter((c) => c.id !== id)
  }

  return {
    customers,
    totalValue,
    wonDealsCount,
    addCustomer,
    deleteCustomer,
  }
})
