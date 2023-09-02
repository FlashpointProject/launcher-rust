import { SuggestionsData, SuggestionsTable } from '../SuggestionsTable';
import React from 'react';
import moment from 'moment';
import { Button, TextField } from '@mui/material';
import { DatePicker, LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterMoment } from '@mui/x-date-pickers/AdapterMoment';

export type GameOfTheDay = {
  id: string,
  author: string,
  description: string,
  date: string,
}

export function Suggestions() {
  const [suggestions, setSuggestions] = React.useState<Array<SuggestionsData>>(
    []
  );
  const [gotds, setGotds] = React.useState<Array<GameOfTheDay>>([]);
  const [selecting, setSelecting] = React.useState<string | null>(null);
  const [date, setDate] = React.useState<string>('');

  React.useEffect(() => {
    fetch('/api/suggestions').then(async (res) => {
      if (res.ok) {
        const suggestions = await res.json();
        setSuggestions(suggestions.suggestions);
      }
    });
  }, []);

  React.useEffect(() => {
    fetch('/api/gotd').then(async (res) => {
      if (res.ok) {
        const gotds = await res.json();
        setGotds(gotds.games);
      }
    });
  }, []);

  const deleteSuggestion = React.useCallback((id: string) => {
    if (confirm(`Are you sure you want to delete ${id}?`)) {
      fetch(`/api/suggestion/${id}`, {
        method: 'DELETE',
      }).then(() => {
        setSuggestions(suggestions.filter((s) => s.id !== id));
      });
    }
  }, [suggestions]);

  const selectSuggestion = React.useCallback((id: string) => {
    setSelecting(id);
  }, []);

  const assignSuggestion = React.useCallback(() => {
    if (selecting && date) {
      const suggestion = suggestions.find((s) => s.id === selecting);
      fetch('/api/gotd', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          date: moment(date).format('YYYY-MM-DD'),
          suggestion,
        }),
      })
        .then(() => {
          setSuggestions(
            suggestions.map((s) => {
              if (s.id === selecting) {
                return {
                  ...s,
                  assigned_dates: [...s.assigned_dates, date],
                };
              } else {
                return s;
              }
            })
          );
          setSelecting(null);
          setDate('');
          // Update taken dates
          const newGotds = [...gotds];
          newGotds.push({
            id: suggestion.game_id,
            author: suggestion.author,
            description: suggestion.description,
            date: moment(date).format('YYYY-MM-DD'),
          });
          setGotds(newGotds);
        })
        .catch((err) => {
          alert(`Error assigning suggestion: ${err}`);
        });
    }
  }, [selecting, date, suggestions]);

  return (
    <div className="suggestions">
      <h1>Suggestions</h1>
      <SuggestionsTable
        deleteSuggestion={deleteSuggestion}
        selectDate={selectSuggestion}
        data={suggestions}
      />
      {selecting && (
        <LocalizationProvider dateAdapter={AdapterMoment}>
          <div className="floating-container__wrapper">
            <div className="floating-container">
              <h2>Select Date</h2>
              <DatePicker
                onChange={(newDate) => {
                  setDate(newDate);
                }}
                value={date}
                shouldDisableDate={(date) => {
                  const compareable = moment(date).format('YYYY-MM-DD');
                  if (gotds.find((g) => g.date === compareable)) {
                    return true;
                  }
                  return false;
                }}
                renderInput={(p) => {
                  return <TextField {...p} />;
                }}
              />
              <Button
                variant="contained"
                color="primary"
                onClick={assignSuggestion}
              >
                Submit
              </Button>
            </div>
          </div>
        </LocalizationProvider>
      )}
    </div>
  );
}
