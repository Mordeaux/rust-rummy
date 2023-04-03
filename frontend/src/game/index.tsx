import { useState } from 'react'
import { useLoaderData } from 'react-router-dom'
import GameViewBase from './GameView'
import { GameState } from '../apiTypes'
import { createContext } from 'react'

type GameStateWithSetter = {
  gameState: GameState | null
  setGameState: (gameState: GameState) => void
}

export const GameStateContext = createContext<GameStateWithSetter>({
  gameState: null,
  setGameState: () => null,
})

export const GameView = () => {
  const data = useLoaderData() as GameState
  const [gameState, setGameState] = useState(data)
  return (
    <GameStateContext.Provider value={{ gameState, setGameState }}>
      <GameViewBase />
    </GameStateContext.Provider>
  )
}

export { default as GameList } from './GameList'
