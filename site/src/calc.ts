import { Stage } from "texas-odds";

export interface OddsOrigin {
  win: number;
  tie: number;
  hand_type_rates: Record<string, number>;
}

export interface Odds {
  win: number;
  tie: number;
  hand_type_rates: Record<string, string>;
}

export function calc(
  players: number,
  holeCardsStr: string,
  community_card: string,
): Odds {
  const holeCards = holeCardsStr.split(" ");
  const stage = new Stage(players, holeCards[0], holeCards[1], community_card);
  const odds = stage.odds() as any as OddsOrigin;
  console.log(odds);
  const output: Odds = {
    win: odds.win,
    tie: odds.win,
    hand_type_rates: {},
  };

  for (const key in odds.hand_type_rates) {
    const rate = odds.hand_type_rates[key] * 100;
    if (rate === 0) {
      output.hand_type_rates[key] = `0`;
    } else if (rate > 0.01) {
      output.hand_type_rates[key] = `${rate.toFixed(2)}%`;
    } else {
      output.hand_type_rates[key] = `${rate.toExponential(4)}%`;
    }
  }
  return output;
}
