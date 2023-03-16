import { useLoaderData, useParams } from 'react-router-dom'
import { draw as apiDraw } from '../api'

const Hand = ({ hand, user = false }) => {
  return (
    <div className="hand">
      {hand.map((card, index) => (
        <h3 key={user ? card : index}>{card}</h3>
      ))}
    </div>
  )
}

const PlayerView = ({ player: { username, score, hand = [] }, isTurn }) => {
  return (
    <div
      className="player-view"
      style={{backgroundColor: isTurn ? 'red' : 'white'}}>
      {username} - {score}
      <Hand hand={hand} />
    </div>
  )
}

const Deck = ({ discards = [] }) => {
  const { gameId } = useParams()
  const draw = () => {
    apiDraw(gameId).then(console.log)
  }

  return (
    <div className="deck">
      <h3>{discards} <span onClick={draw.bind(undefined)}>The Deck</span></h3>
    </div>
  )
}

const GameView = () => {
  const { name: gameName, players, discards, turn } = useLoaderData()
  const opponent = players.find(({ userId }) => userId !== 1)
  const player = players.find(({ userId }) => userId === 1)
  players.sort(player => player.order_index)
  console.log(players)
  console.log(turn)

  return (
    <div className="game-view">
      <h1>{gameName}</h1>
      <div className="game-board">
        <PlayerView player={opponent} isTurn={opponent === players[turn]} />
        <Deck discards={discards} />
        <PlayerView player={player} user isTurn={player === players[turn]} />
        <h2>DISCARD</h2>
      </div>
    </div>
  )
}

export default GameView
