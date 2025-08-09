import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import { SorobanRpc, Networks } from 'soroban-client'

interface SorobanContextType {
  server: SorobanRpc
  network: string
  isConnected: boolean
  connect: () => Promise<void>
  disconnect: () => void
}

const SorobanContext = createContext<SorobanContextType | undefined>(undefined)

export const useSoroban = () => {
  const context = useContext(SorobanContext)
  if (context === undefined) {
    throw new Error('useSoroban must be used within a SorobanProvider')
  }
  return context
}

interface SorobanProviderProps {
  children: ReactNode
}

export const SorobanProvider: React.FC<SorobanProviderProps> = ({ children }) => {
  const [server, setServer] = useState<SorobanRpc>(new SorobanRpc('https://soroban-rpc.stellar.org'))
  const [network, setNetwork] = useState<string>('testnet')
  const [isConnected, setIsConnected] = useState<boolean>(false)

  const connect = async () => {
    try {
      // For demo purposes, we'll use testnet
      const testnetServer = new SorobanRpc('https://soroban-testnet.stellar.org')
      setServer(testnetServer)
      setNetwork('testnet')
      setIsConnected(true)
    } catch (error) {
      console.error('Failed to connect to Soroban:', error)
      setIsConnected(false)
    }
  }

  const disconnect = () => {
    setIsConnected(false)
  }

  useEffect(() => {
    // Auto-connect on mount
    connect()
  }, [])

  const value: SorobanContextType = {
    server,
    network,
    isConnected,
    connect,
    disconnect,
  }

  return (
    <SorobanContext.Provider value={value}>
      {children}
    </SorobanContext.Provider>
  )
}
