import { useLoaderData, useParams } from 'react-router-dom'
import { draw as apiDraw } from '../api'
import wasmPromise from '../rummy-wasm'
import { Card, Player, GameState } from '../apiTypes'

const Hand = ({ hand, user = false }: {hand: Card[], user?: Boolean}) => {
  const displayCard = ({ card }: Card) => (card ? `${card.rank}${card.suit}` : 'xx')
  return (
    <div className="hand">
      {hand.map((card, index) => (
        <h3 key={user ? displayCard(card) : index}>{displayCard(card)}</h3>
      ))}
    </div>
  )
}

const PlayerView = ({ player: { username, score, hand = [] }, isTurn }: {player: Player, isTurn:Boolean}) => {
  return (
    <div
      className="player-view"
      style={{ backgroundColor: isTurn ? 'red' : 'white' }}
    >
      {username} - {score}
      <Hand hand={hand} />
    </div>
  )
}

const Deck = ({ discards = [] }: {discards: Card[]}) => {
  const { gameId } = useParams()
  const data = useLoaderData()
  wasmPromise.then((wasm) =>
    console.log(wasm.getAvailableMoves(JSON.stringify(data)))
  )
  const draw = () => {
    apiDraw(gameId).then(console.log)
  }

  return (
    <div className="deck">
      <h3>
        {discards.map(({card}: Card) => `${card.rank}${card.suit}`)}{' '}
        <span onClick={draw.bind(undefined)}>The Deck</span>
      </h3>
    </div>
  )
}

const GameView = () => {
  const {
    name: gameName,
    players,
    discards,
    current_turn: currentTurn,
  } = useLoaderData() as GameState
  const opponent = players.find(({ id }) => id !== 1) as Player
  const player = players.find(({ id }) => id === 1) as Player
  players.sort((player) => player.order_index)

  return (
    <div className="game-view">
      <h1>{gameName}</h1>
      <div className="game-board">
        <PlayerView
          player={opponent}
          isTurn={opponent === players[currentTurn]}
        />
        <Deck discards={discards} />
        <PlayerView
          player={player}
          isTurn={player === players[currentTurn]}
        />
        <h2>DISCARD</h2>
      </div>
    </div>
  )
}

export default GameView
