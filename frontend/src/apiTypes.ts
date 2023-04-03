export type User = {
  id: number
  username: string
}

export type Player = User & {
  score: number
  order_index: number
  melds: Array<Array<Card>>
  hand: Card[]
}

export type Card = {
  card_type: 'visible' | 'hidden'
  card: {
    rank: number
    suit: 'spades' | 'diamonds' | 'clubs' | 'hearts'
    playable: boolean
  }
}

type TurnPhase = 'WAIT_FOR_TURN' | 'DRAW' | 'PLAY'

export type DrawPhaseMove =
  | (Card & {
      move_type: 'draw_from_discards'
    })
  | { move_type: 'draw_from_deck'; card_type: 'hidden' }

type AvailableMoves = {
  draw: DrawPhaseMove[]
}

export type GameState = {
  id: number
  players: Player[]
  current_turn: number
  turn_phase: TurnPhase
  discards: Card[]
  name: string
  available_moves: AvailableMoves
}
