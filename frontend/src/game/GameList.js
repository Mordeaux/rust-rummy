import { useLoaderData } from 'react-router-dom'

const GameList = () => {
  const { user, games = [] } = useLoaderData()
  const filterOutSelf = ({ userId }) => userId !== user.userId

  return (
    <ul>
      {games.map(
        game => (
          <li key={game.id}>
            {game.name} with {game.users.filter(filterOutSelf).map(user => user.username)}
          </li>
        )
      )}
    </ul>
  )
}

export default GameList
