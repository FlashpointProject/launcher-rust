import { SuggestionsData, SuggestionsTable } from '../SuggestionsTable';
import React from 'react';

export function Suggestions() {
  const [suggestions, setSuggestions] = React.useState<Array<SuggestionsData>>([]);

  React.useEffect(() => {
    fetch('/api/suggestions')
      .then(async (res) => {
        if (res.ok) {
          const suggestions = await res.json();
          setSuggestions(suggestions.suggestions);
        }
      });
  }, []);

  const deleteSuggestion = React.useCallback((id: string) => {
    fetch(`/api/suggestion/${id}`, {
      method: 'DELETE',
    })
      .then(() => {
        setSuggestions(suggestions.filter((s) => s.id !== id));
      });
  }, [suggestions]);

  return (
    <div className='suggestions'>
      <h1>Suggestions</h1>
      <SuggestionsTable deleteSuggestion={deleteSuggestion} data={suggestions} />
    </div>
  );
}