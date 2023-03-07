import { useLoaderData } from 'react-router-dom'

const Hand = ({ hand, user = false }) => {
  return (
    <div className="hand">
      {hand.map((card, index) => (
        <h3 key={user ? card : index}>{card}</h3>
      ))}
    </div>
  )
}

const PlayerView = ({ player: { username, score, hand = [] } }) => {
  return (
    <div className="player-view">
      {username} - {score}
      <Hand hand={hand} />
    </div>
  )
}

const GameView = () => {
  const { name: gameName, players } = useLoaderData()
  const opponent = players.find(({ userId }) => userId !== 1)
  const player = players.find(({ userId }) => userId === 1)

  return (
    <div className="game-view">
      <h1>{gameName}</h1>
      <div className="game-board">
        <PlayerView player={opponent} />
        <PlayerView player={player} user />
      </div>
    </div>
  )
}

export default GameView
