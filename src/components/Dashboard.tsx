import React, { useState, useEffect } from 'react'
import { 
  TrendingUp, 
  Users, 
  Bike, 
  DollarSign, 
  Leaf, 
  MapPin, 
  Clock,
  Award,
  Target,
  Activity
} from 'lucide-react'
import { LineChart, Line, AreaChart, Area, BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, PieChart, Pie, Cell } from 'recharts'
import { useSoroban } from '../contexts/SorobanContext'

// Mock data for demo purposes - in real implementation this would come from Soroban contracts
const mockData = {
  totalAssets: 15,
  totalInvestors: 247,
  totalFunding: 125000,
  co2Saved: 2847,
  underservedRides: 1234,
  equityScore: 78,
  revenueDistributed: 45600,
  activeProposals: 3,
  impactMetrics: [
    { month: 'Jan', rides: 1200, co2: 240, revenue: 8000 },
    { month: 'Feb', rides: 1350, co2: 270, revenue: 9000 },
    { month: 'Mar', rides: 1500, co2: 300, revenue: 10000 },
    { month: 'Apr', rides: 1650, co2: 330, revenue: 11000 },
    { month: 'May', rides: 1800, co2: 360, revenue: 12000 },
    { month: 'Jun', rides: 1950, co2: 390, revenue: 13000 },
  ],
  assetTypes: [
    { name: 'E-Bikes', value: 8, color: '#3b82f6' },
    { name: 'Shuttles', value: 4, color: '#10b981' },
    { name: 'Scooters', value: 3, color: '#f59e0b' },
  ],
  equityDistribution: [
    { zone: 'Low Income', equity: 85, rides: 45 },
    { zone: 'Middle Income', equity: 65, rides: 35 },
    { zone: 'High Income', equity: 45, rides: 20 },
  ]
}

const Dashboard: React.FC = () => {
  const { isConnected } = useSoroban()
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    // Simulate loading time
    const timer = setTimeout(() => {
      setIsLoading(false)
    }, 1000)

    return () => clearTimeout(timer)
  }, [])

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600"></div>
      </div>
    )
  }

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="text-center">
        <h1 className="text-4xl font-bold text-gray-900 mb-4">
          AI-Enhanced Equitable Urban Mobility Finance
        </h1>
        <p className="text-xl text-gray-600 max-w-3xl mx-auto">
          Real-time impact dashboard showing how AI-driven equitable finance is transforming urban mobility 
          and reducing transportation inequities in underserved communities.
        </p>
      </div>

      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-primary-600">Total Assets Funded</p>
              <p className="text-3xl font-bold text-primary-900">{mockData.totalAssets}</p>
            </div>
            <Bike className="w-8 h-8 text-primary-600" />
          </div>
          <div className="mt-4 flex items-center text-sm text-primary-700">
            <TrendingUp className="w-4 h-4 mr-1" />
            <span>+12% from last month</span>
          </div>
        </div>

        <div className="equity-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-equity-600">Equity Score</p>
              <p className="text-3xl font-bold text-equity-900">{mockData.equityScore}%</p>
            </div>
            <Award className="w-8 h-8 text-equity-600" />
          </div>
          <div className="mt-4 flex items-center text-sm text-equity-700">
            <Target className="w-4 h-4 mr-1" />
            <span>AI-optimized for fairness</span>
          </div>
        </div>

        <div className="impact-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-success-600">CO₂ Saved (kg)</p>
              <p className="text-3xl font-bold text-success-900">{mockData.co2Saved.toLocaleString()}</p>
            </div>
            <Leaf className="w-8 h-8 text-success-600" />
          </div>
          <div className="mt-4 flex items-center text-sm text-success-700">
            <Activity className="w-4 h-4 mr-1" />
            <span>Equivalent to {Math.round(mockData.co2Saved / 22)} trees</span>
          </div>
        </div>

        <div className="card">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-600">Total Funding</p>
              <p className="text-3xl font-bold text-gray-900">${mockData.totalFunding.toLocaleString()}</p>
            </div>
            <DollarSign className="w-8 h-8 text-gray-600" />
          </div>
          <div className="mt-4 flex items-center text-sm text-gray-700">
            <Users className="w-4 h-4 mr-1" />
            <span>{mockData.totalInvestors} community investors</span>
          </div>
        </div>
      </div>

      {/* Charts Section */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Growth Trends */}
        <div className="card">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Monthly Growth Trends</h3>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={mockData.impactMetrics}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="month" />
              <YAxis />
              <Tooltip />
              <Line type="monotone" dataKey="rides" stroke="#3b82f6" strokeWidth={2} name="Rides" />
              <Line type="monotone" dataKey="revenue" stroke="#10b981" strokeWidth={2} name="Revenue" />
            </LineChart>
          </ResponsiveContainer>
        </div>

        {/* Asset Distribution */}
        <div className="card">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Asset Type Distribution</h3>
          <ResponsiveContainer width="100%" height={300}>
            <PieChart>
              <Pie
                data={mockData.assetTypes}
                cx="50%"
                cy="50%"
                outerRadius={80}
                fill="#8884d8"
                dataKey="value"
                label={({ name, value }) => `${name}: ${value}`}
              >
                {mockData.assetTypes.map((entry, index) => (
                  <Cell key={`cell-${index}`} fill={entry.color} />
                ))}
              </Pie>
              <Tooltip />
            </PieChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Equity Impact */}
      <div className="card">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Equity Impact by Zone</h3>
        <ResponsiveContainer width="100%" height={300}>
          <BarChart data={mockData.equityDistribution}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="zone" />
            <YAxis />
            <Tooltip />
            <Bar dataKey="equity" fill="#d946ef" name="Equity Score (%)" />
            <Bar dataKey="rides" fill="#3b82f6" name="Ride Share (%)" />
          </BarChart>
        </ResponsiveContainer>
      </div>

      {/* Impact Highlights */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="card">
          <div className="flex items-center space-x-3 mb-4">
            <div className="w-10 h-10 bg-success-100 rounded-lg flex items-center justify-center">
              <MapPin className="w-5 h-5 text-success-600" />
            </div>
            <div>
              <h4 className="font-semibold text-gray-900">Underserved Communities</h4>
              <p className="text-sm text-gray-600">Enhanced access</p>
            </div>
          </div>
          <p className="text-2xl font-bold text-success-600">{mockData.underservedRides}</p>
          <p className="text-sm text-gray-600">rides in underserved areas</p>
        </div>

        <div className="card">
          <div className="flex items-center space-x-3 mb-4">
            <div className="w-10 h-10 bg-primary-100 rounded-lg flex items-center justify-center">
              <DollarSign className="w-5 h-5 text-primary-600" />
            </div>
            <div>
              <h4 className="font-semibold text-gray-900">Revenue Distributed</h4>
              <p className="text-sm text-gray-600">To investors</p>
            </div>
          </div>
          <p className="text-2xl font-bold text-primary-600">${mockData.revenueDistributed.toLocaleString()}</p>
          <p className="text-sm text-gray-600">with equity bonuses</p>
        </div>

        <div className="card">
          <div className="flex items-center space-x-3 mb-4">
            <div className="w-10 h-10 bg-warning-100 rounded-lg flex items-center justify-center">
              <Clock className="w-5 h-5 text-warning-600" />
            </div>
            <div>
              <h4 className="font-semibold text-gray-900">Active Proposals</h4>
              <p className="text-sm text-gray-600">Community governance</p>
            </div>
          </div>
          <p className="text-2xl font-bold text-warning-600">{mockData.activeProposals}</p>
          <p className="text-sm text-gray-600">awaiting community vote</p>
        </div>
      </div>

      {/* AI Integration Highlight */}
      <div className="bg-gradient-to-r from-purple-50 to-pink-50 border border-purple-200 rounded-xl p-8">
        <div className="text-center">
          <h3 className="text-2xl font-bold text-gray-900 mb-4">
            🤖 AI-Powered Equitable Finance
          </h3>
          <p className="text-lg text-gray-700 mb-6 max-w-2xl mx-auto">
            Our platform uses AI oracles to dynamically adjust loan rates and revenue distribution 
            based on urban equity data, ensuring fair access to mobility solutions for all communities.
          </p>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 text-sm">
            <div className="text-center">
              <div className="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mx-auto mb-2">
                <Target className="w-6 h-6 text-purple-600" />
              </div>
              <p className="font-semibold text-gray-900">Equity Scoring</p>
              <p className="text-gray-600">AI analyzes urban data for fair rates</p>
            </div>
            <div className="text-center">
              <div className="w-12 h-12 bg-pink-100 rounded-lg flex items-center justify-center mx-auto mb-2">
                <TrendingUp className="w-6 h-6 text-pink-600" />
              </div>
              <p className="font-semibold text-gray-900">Dynamic Adjustments</p>
              <p className="text-gray-600">Real-time rate optimization</p>
            </div>
            <div className="text-center">
              <div className="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mx-auto mb-2">
                <Award className="w-6 h-6 text-purple-600" />
              </div>
              <p className="font-semibold text-gray-900">Impact Rewards</p>
              <p className="text-gray-600">Bonuses for underserved areas</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default Dashboard
