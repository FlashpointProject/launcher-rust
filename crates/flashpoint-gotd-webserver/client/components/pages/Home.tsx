import React from 'react';
import Checkbox from '@mui/material/Checkbox';
import TextField from '@mui/material/TextField';
import { Game } from '../../types';
import { GameBox } from '../GameBox';
import { plainToInstance } from 'class-transformer';
import { Button } from '@mui/material';
import { UserContext } from '../app';

export function Home() {
  const [gameId, setGameId] = React.useState('');
  const [anonymous, setAnonymous] = React.useState(false);
  const [description, setDescription] = React.useState('');
  const [game, setGame] = React.useState<Game | null>(null);
  const user = React.useContext(UserContext);

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
      {user && user.authenticated ? (
        <>
          <h1>GOTD Suggestion Form</h1>
          <p>Logged in as {user.username}</p>
          <p>Enter a game ID to get its info:</p>
          <table className='formTable'>
            <tbody>
              <tr>
                <td>Game ID: </td>
                <td>
                  <TextField onChange={(event) => {
                    console.log('change to ' + event.target.value);
                    setGameId(event.target.value);
                  }}></TextField>
                </td>
              </tr>
              <tr>
                <td>Anonymous: </td>
                <td>
                  <Checkbox onChange={(event) => {
                    setAnonymous(event.target.checked);
                  }}></Checkbox>
                </td>
              </tr>
              <tr>
                <td>Description: </td>
                <td>
                  <TextField multiline={true} minRows={3} onChange={(event) => {
                    setDescription(event.target.value);
                  }}></TextField>
                </td>
              </tr>
            </tbody>
          </table>
          {game && (
            <>
              <GameBox game={game}></GameBox>
              {description && (
                <Button
                  onClick={postSuggestion}
                  variant="contained">
                  Submit Suggestion
                </Button>
              )}
            </>
          )}
        </>
      ) : (
        <>
          <p>You must be logged in to post suggestions.</p>
          <Button
            href="/api/auth/login"
            variant="contained">
            Login
          </Button>
        </>
      )}

    </div >
  );
}
