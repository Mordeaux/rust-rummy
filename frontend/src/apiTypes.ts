
export type User = {
  id: number,
  username: string,
}

export type Player = User & {
  score: number,
  order_index: number,
  melds: Array<Array<Card>>,
  hand: Card[],
}

export type Card = {
  type: 'OwnCard' | 'OpponentCard' | 'DiscardCard' | 'MeldCard',
card: {
  rank: number,
  suit: 'spades' | 'diamonds' | 'clubs' | 'hearts',
},
  }

type TurnPhase = 'WAIT_FOR_TURN' | 'DRAW' | 'PLAY'

export type GameState = {
  id: number,
  players: Player[],
  current_turn: number,
  turn_phase: TurnPhase,
  discards: Card[],
  name: string,
}