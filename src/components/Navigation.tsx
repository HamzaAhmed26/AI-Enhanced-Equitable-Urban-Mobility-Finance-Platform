import React from 'react'
import { Bike, Users, TrendingUp, Zap } from 'lucide-react'
import { useSoroban } from '../contexts/SorobanContext'

const Navigation: React.FC = () => {
  const { isConnected, network } = useSoroban()

  return (
    <nav className="bg-white shadow-sm border-b border-gray-200">
      <div className="container mx-auto px-4">
        <div className="flex justify-between items-center h-16">
          {/* Logo and Brand */}
          <div className="flex items-center space-x-3">
            <div className="flex items-center justify-center w-10 h-10 bg-gradient-to-br from-primary-500 to-primary-600 rounded-lg">
              <Bike className="w-6 h-6 text-white" />
            </div>
            <div>
              <h1 className="text-xl font-bold text-gray-900">Urban Mobility Finance</h1>
              <p className="text-sm text-gray-500">AI-Enhanced Equitable Platform</p>
            </div>
          </div>

          {/* Navigation Links */}
          <div className="hidden md:flex items-center space-x-8">
            <a href="#dashboard" className="flex items-center space-x-2 text-gray-700 hover:text-primary-600 transition-colors">
              <TrendingUp className="w-4 h-4" />
              <span>Dashboard</span>
            </a>
            <a href="#assets" className="flex items-center space-x-2 text-gray-700 hover:text-primary-600 transition-colors">
              <Bike className="w-4 h-4" />
              <span>Assets</span>
            </a>
            <a href="#community" className="flex items-center space-x-2 text-gray-700 hover:text-primary-600 transition-colors">
              <Users className="w-4 h-4" />
              <span>Community</span>
            </a>
            <a href="#impact" className="flex items-center space-x-2 text-gray-700 hover:text-primary-600 transition-colors">
              <Zap className="w-4 h-4" />
              <span>Impact</span>
            </a>
          </div>

          {/* Network Status */}
          <div className="flex items-center space-x-4">
            <div className="flex items-center space-x-2">
              <div className={`w-2 h-2 rounded-full ${isConnected ? 'bg-success-500' : 'bg-red-500'}`}></div>
              <span className="text-sm text-gray-600">
                {isConnected ? `${network.toUpperCase()}` : 'Disconnected'}
              </span>
            </div>
            
            {/* Hackathon Badge */}
            <div className="bg-gradient-to-r from-purple-500 to-pink-500 text-white px-3 py-1 rounded-full text-xs font-medium">
              UK AI Hackathon
            </div>
          </div>
        </div>
      </div>
    </nav>
  )
}

export default Navigation
