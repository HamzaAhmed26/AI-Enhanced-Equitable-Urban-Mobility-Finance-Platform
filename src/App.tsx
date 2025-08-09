import React from 'react'
import { Routes, Route } from 'react-router-dom'
import Dashboard from './components/Dashboard'
import Navigation from './components/Navigation'
import { SorobanProvider } from './contexts/SorobanContext'

function App() {
  return (
    <SorobanProvider>
      <div className="min-h-screen bg-gray-50">
        <Navigation />
        <main className="container mx-auto px-4 py-8">
          <Routes>
            <Route path="/" element={<Dashboard />} />
          </Routes>
        </main>
      </div>
    </SorobanProvider>
  )
}

export default App
