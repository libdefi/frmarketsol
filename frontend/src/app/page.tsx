'use client'

import { useState, useEffect } from 'react'
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Card, CardContent } from "@/components/ui/card"
import { AspectRatio } from "@/components/ui/aspect-ratio"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog"

// Define the type
type Bet = {
    prediction: string; // or whatever type prediction should be
    // ... other properties if needed
};

export default function Home() {
  const [newBet, setNewBet] = useState('')
  const [countdown, setCountdown] = useState('')
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [betAmount, setBetAmount] = useState('')
  const [selectedBet, setSelectedBet] = useState<Bet | null>(null); 

  useEffect(() => {
    const targetDate = new Date()
    targetDate.setDate(targetDate.getDate() + 7)

    const timer = setInterval(() => {
      const now = new Date()
      const difference = targetDate.getTime() - now.getTime()

      const days = Math.floor(difference / (1000 * 60 * 60 * 24))
      const hours = Math.floor((difference % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60))
      const minutes = Math.floor((difference % (1000 * 60 * 60)) / (1000 * 60))
      const seconds = Math.floor((difference % (1000 * 60)) / 1000)

      setCountdown(`${days}d ${hours}h ${minutes}m ${seconds}s`)

      if (difference < 0) {
        clearInterval(timer)
        setCountdown('Betting closed')
      }
    }, 1000)

    return () => clearInterval(timer)
  }, [])

  const predictions = [
    { user: '0xzara.eth', time: '52m ago', prediction: 'Taylor Swift - "Cruel Summer" (Official Video)', amount: '$23,214', videoId: 'ic8j13piAhQ' },
    { user: '0xsmallbrain', time: '1h ago', prediction: 'The Weeknd - "Blinding Lights" (Official Music Video)', amount: '$23,214', videoId: '4NRXx6U8ABQ' },
    { user: 'moyed', time: '1h ago', prediction: 'BTS - "Butter" (Official Music Video)', amount: '$23,214', videoId: 'WMweEpGlu_U' },
    { user: 'musicfan99', time: '2h ago', prediction: 'Dua Lipa - "Levitating" ft. DaBaby (Official Music Video)', amount: '$18,750', videoId: 'TUVcZfQe-Kw' },
    { user: 'charttopper', time: '3h ago', prediction: 'Billie Eilish - "bad guy" (Official Music Video)', amount: '$15,980', videoId: 'DyDfgMOUjCI' },
    { user: 'popculture101', time: '4h ago', prediction: 'Ariana Grande - "7 rings" (Official Music Video)', amount: '$14,325', videoId: 'QYh6mYIJG2Y' },
    { user: 'hiphophead', time: '5h ago', prediction: 'Drake - "God\'s Plan" (Official Music Video)', amount: '$12,750', videoId: 'xpVfcZ0ZcFM' },
    { user: 'rocklegend', time: '6h ago', prediction: 'Imagine Dragons - "Believer" (Official Music Video)', amount: '$11,500', videoId: '7wtfhZwyrcc' },
    { user: 'indievibes', time: '7h ago', prediction: 'Tame Impala - "The Less I Know The Better" (Official Music Video)', amount: '$10,200', videoId: 'sBzrzS1Ag_g' },
    { user: 'latinobeats', time: '8h ago', prediction: 'Bad Bunny - "Yo Perreo Sola" (Official Music Video)', amount: '$9,800', videoId: 'GtSRKwDCaZM' },
    { user: 'edmlover', time: '9h ago', prediction: 'Calvin Harris, Dua Lipa - "One Kiss" (Official Music Video)', amount: '$8,950', videoId: 'DkeiKbqa02g' },
    { user: 'soulfulnotes', time: '10h ago', prediction: 'H.E.R. - "Damage" (Official Music Video)', amount: '$7,500', videoId: 'PAFAfhod9TU' },
  ]

  const handlePlaceBet = (bet: { user: string; time: string; prediction: string; amount: string; videoId: string; }) => {
    const betObject: Bet = { 
        prediction: bet.prediction, // Assuming 'bet' is the prediction
        // ... include other required properties of Bet here
    };
    setSelectedBet(betObject);
    setIsModalOpen(true);
  }

  const handleConfirmBet = () => {
    // Here you would typically handle the bet confirmation
    // For now, we'll just close the modal and reset the bet amount
    setIsModalOpen(false)
    setBetAmount('')
    // You might want to update the UI to reflect the new bet
  }

  return (
    <div className="max-w-6xl mx-auto p-4">
      <header className="flex justify-between items-center mb-8">
        <h1 className="text-2xl font-bold">Prediction.App</h1>
        <div className="flex items-center">
          <Button variant="ghost" size="sm" className="mr-6">How it works</Button>
          <Avatar>
            <AvatarImage src="https://github.com/shadcn.png" alt="@shadcn" />
            <AvatarFallback>CN</AvatarFallback>
          </Avatar>
        </div>
      </header>

      <main>
        <h2 className="text-4xl font-bold text-center mb-2">
          Predict Next Week&apos;s Trending Music on YouTube
        </h2>
        <div className="flex items-center justify-center space-x-2 mb-6">
          <div className="w-2 h-2 rounded-full bg-red-500 animate-pulse" aria-hidden="true"></div>
          <div className="text-red-500 font-semibold" aria-live="polite" aria-atomic="true">{countdown}</div>
        </div>
        <div className="mb-8 flex space-x-2">
          <Input
            type="text"
            placeholder="Enter YouTube link to create new bet"
            value={newBet}
            onChange={(e) => setNewBet(e.target.value)}
            className="flex-grow"
          />
          <Button variant="default">Create</Button>
        </div>

        <div className="space-y-6">
          <div className="flex justify-end text-sm text-gray-500">
            <span>Total Predictions: {predictions.length} | Total Volume: $179,397</span>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            {predictions.map((bet, index) => (
              <Card key={index} className="overflow-hidden">
                <AspectRatio ratio={16 / 9}>
                  <img
                    src={`https://img.youtube.com/vi/${bet.videoId}/0.jpg`}
                    alt={bet.prediction}
                    className="object-cover w-full h-full"
                  />
                </AspectRatio>
                <CardContent className="p-4">
                  <div className="flex items-center space-x-2 mb-2">
                    <Avatar className="w-6 h-6">
                      <AvatarFallback>{bet.user[0].toUpperCase()}</AvatarFallback>
                    </Avatar>
                    <span className="font-medium text-sm">{bet.user}</span>
                    <span className="text-gray-500 text-xs">{bet.time}</span>
                  </div>
                  <h3 className="font-semibold mb-2 line-clamp-2">{bet.prediction}</h3>
                  <div className="flex justify-between items-center">
                    <span className="text-green-500 font-semibold">{bet.amount}</span>
                    <Button variant="outline" size="sm" onClick={() => handlePlaceBet(bet)}>Place Bet</Button>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        </div>
      </main>

      <Dialog open={isModalOpen} onOpenChange={setIsModalOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Place Your Bet</DialogTitle>
          </DialogHeader>
          <div className="py-4">
            <h3 className="font-semibold mb-2">{selectedBet?.prediction}</h3>
            <Input
              type="number"
              placeholder="Enter bet amount in USD"
              value={betAmount}
              onChange={(e) => setBetAmount(e.target.value)}
              className="mb-4"
            />
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsModalOpen(false)}>Cancel</Button>
            <Button onClick={handleConfirmBet}>Confirm Bet</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  )
}
