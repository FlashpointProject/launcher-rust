import moment from 'moment';
import React from 'react';
import { GotdTable } from '../GotdTable';
import { GameOfTheDay } from './Suggestions';

export function Gotds() {
  const [gotds, setGotds] = React.useState<Array<GameOfTheDay>>([]);
  React.useEffect(() => {
    fetch('/api/gotd').then(async (res) => {
      if (res.ok) {
        const gotds = await res.json();
        setGotds(gotds.games.sort((a: GameOfTheDay, b: GameOfTheDay) => moment(a.date).diff(moment(b.date))));
      }
    });
  }, []);

  const deleteGotd = React.useCallback((date: string) => {
    if (confirm(`Are you sure you want to delete ${date}?`)) {
      fetch(`/api/gotd/${date}`, {
        method: 'DELETE',
      }).then(() => {
        setGotds(gotds.filter((s) => s.date !== date));
      });
    }
  }, [gotds]);

  return (
    <div>
      <h1>Game of the Day List</h1>
      {gotds && (
        <GotdTable data={gotds} deleteGotd={deleteGotd} />
      )}
    </div>
  );
}