import React from 'react';
import { Game } from '../../types';
import { GameBox } from '../GameBox';
import { plainToInstance } from 'class-transformer';

export function Home() {
  const [gameId, setGameId] = React.useState('');
  const [author, setAuthor] = React.useState('');
  const [description, setDescription] = React.useState('');
  const [game, setGame] = React.useState<Game | null>(null);

  const postSuggestion = React.useCallback(() => {
    if (game && description) {
      const body = {
        id: gameId,
        title: game.title,
        author: author || undefined,
        description,
      };
      fetch('/api/suggestion', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Acceot': 'application/json'
        },
        body: JSON.stringify(body),
      })
        .then(() => {
          alert('Suggestion submitted!');
        })
        .catch((err) => { alert(`Error submitting suggestion: ${err}`); });
    }
  }, [gameId, author, description, game]);

  React.useEffect(() => {
    async function fetchGame(gameId: string) {
      if (gameId.length === 36) {
        const res = await fetch(`/api/game/${gameId}`);
        if (res.ok) {
          const json = await res.json();
          const game = plainToInstance(Game, json);
          return game;
        }
        return null;
      } else {
        return null;
      }
    }
    fetchGame(gameId)
      .then((game) => {
        setGame(game);
      });
  }, [gameId]);

  return (
    <div>
      <h1>Flashpoint GOTD Server</h1>
      <p>Enter a game ID to get its info:</p>
      <table>
        <tr>
          <td>Game ID: </td>
          <td>
            <input onChange={(event) => {
              setGameId(event.target.value);
            }}></input>
          </td>
        </tr>
        <tr>
          <td>Author: </td>
          <td>
            <input onChange={(event) => {
              setAuthor(event.target.value);
            }}></input>
          </td>
        </tr>
        <tr>
          <td>Description: </td>
          <td>
            <input onChange={(event) => {
              setDescription(event.target.value);
            }}></input>
          </td>
        </tr>
      </table>
      {game && (
        <>
          <GameBox game={game}></GameBox>
          {description && (
            <button onClick={postSuggestion}>Submit Suggestion</button>
          )}
        </>
      )}
    </div>
  );
}
