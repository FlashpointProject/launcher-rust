import React from 'react';
import { Game } from '../../types';
import { GameBox } from '../GameBox';
import { plainToInstance } from 'class-transformer';

export function Home() {
  const [gameId, setGameId] = React.useState('');
  const [anonymous, setAnonymous] = React.useState(false);
  const [description, setDescription] = React.useState('');
  const [game, setGame] = React.useState<Game | null>(null);
  const [username, setUsername] = React.useState<string | undefined>(undefined);

  React.useEffect(() => {
    fetch('/api/auth/info')
      .then(async (res) => {
        if (res.ok) {
          const identity = await res.json();
          console.log(identity);
          setUsername(identity.username);
        }
      });
  }, []);

  const postSuggestion = React.useCallback(() => {
    if (game && description) {
      const body = {
        id: gameId,
        title: game.title,
        anonymous,
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
  }, [gameId, anonymous, description, game]);

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
      {username ? (
        <>
          <p>Logged in as {username}</p>
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
              <td>Anonymous: </td>
              <td>
                <input type="checkbox" onChange={(event) => {
                  setAnonymous(event.target.checked);
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
        </>
      ) : (
        <button>
          <a href="/api/auth/login">Login</a>
        </button>
      )}

    </div >
  );
}
