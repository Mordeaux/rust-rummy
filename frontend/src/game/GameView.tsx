import { useParams } from 'react-router-dom'
import { GameStateContext } from './index'
import { putMove } from '../api'
import { Card, Player } from '../apiTypes'

const Hand = ({ hand, user = false }: { hand: Card[]; user?: Boolean }) => {
  const displayCard = ({ card }: Card) =>
    card ? `${card.rank}${card.suit}` : 'xx'
  return (
    <div className="hand">
      {hand.map((card, index) => (
        <h3 key={user ? displayCard(card) : index}>{displayCard(card)}</h3>
      ))}
    </div>
  )
}

const PlayerView = ({
  player: { username, score, hand = [] },
  isTurn,
}: {
  player: Player
  isTurn: Boolean
}) => {
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

const DiscardCard = ({ card }: Card) => <>{`${card.rank}${card.suit}`}</>

const GameView = () => (
  <div className="game-view">
    <GameStateContext.Consumer>
      {({ gameState, setGameState }) => {
        if (gameState) {
          const {
            id,
            name: gameName,
            players,
            discards,
            current_turn: currentTurn,
          } = gameState
          const opponent = players.find(({ id }) => id !== 1) as Player
          const player = players.find(({ id }) => id === 1) as Player
          const drawFromDeck = () =>
            putMove(`${id}`, {
              move_type: 'draw_from_deck',
              card_type: 'hidden',
            }).then(setGameState)
          const drawFromDiscards = ({ card }: Card) =>
            putMove(`${id}`, {
              move_type: 'draw_from_discards',
              card_type: 'visible',
              card,
            }).then(setGameState)

          return (
            <>
              <h1>{gameName}</h1>
              <div className="game-board">
                <PlayerView
                  player={opponent}
                  isTurn={opponent.order_index === currentTurn}
                />
                <div className="deck">
                  <h3>
                    {discards.map((card: Card) => (
                      <div onClick={() => drawFromDiscards(card)}>
                        <DiscardCard {...card} />
                      </div>
                    ))}{' '}
                    <span onClick={drawFromDeck}>The Deck</span>
                  </h3>
                </div>
                <PlayerView
                  player={player}
                  isTurn={player.order_index === currentTurn}
                />
                <h2>DISCARD</h2>
              </div>
            </>
          )
        }
      }}
    </GameStateContext.Consumer>
  </div>
)

export default GameView
