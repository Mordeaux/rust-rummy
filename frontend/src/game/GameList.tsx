import { Link } from 'react-router-dom'
import { User, GameState } from '../apiTypes'

const GameList = ({ user, games }: { user: User; games: GameState[] }) => {
  const filterOutSelf = ({ id }: User) => id !== user.id

  return (
    <ul>
      {games.map((game) => (
        <li key={game.id}>
          <Link to={`/games/${game.id}`}>
            {game.name} with{' '}
            {game.players.filter(filterOutSelf).map((user) => user.username)}
          </Link>
        </li>
      ))}
    </ul>
  )
}

export default GameList
