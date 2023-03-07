import { useLoaderData, Link } from 'react-router-dom'

const GameList = () => {
  const { user, games = [] } = useLoaderData()
  const filterOutSelf = ({ userId }) => userId !== user.userId

  return (
    <ul>
      {games.map((game) => (
        <li key={game.id}>
          <Link to={`/games/${game.id}`}>
            {game.name} with{' '}
            {game.users.filter(filterOutSelf).map((user) => user.username)}
          </Link>
        </li>
      ))}
    </ul>
  )
}

export default GameList
