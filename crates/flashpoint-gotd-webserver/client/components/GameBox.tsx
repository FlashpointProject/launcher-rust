import { Game } from '../types';

type GameBoxProps = {
  game: Game
}

export function GameBox(props: GameBoxProps) {
  return (
    <div>
      <h2>{props.game.title}</h2>
    </div>
  );
}
