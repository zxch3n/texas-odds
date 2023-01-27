import { Stage } from "texas-odds";

export interface Odds {
  win: number;
  tie: number;
  hand_type_rates: Record<string, number>;
}

export function calc(
  players: number,
  holeCardsStr: string,
  community_card: string,
): Odds {
  const holeCards = holeCardsStr.split(" ");
  const stage = new Stage(players, holeCards[0], holeCards[1], community_card);
  const odds = stage.odds() as any as Odds;
  console.log(odds);
  return odds;
}
