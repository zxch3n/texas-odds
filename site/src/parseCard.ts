export interface Card {
  suit: "♠" | "♥" | "♦" | "♣";
  num: number;
}

export function parseCards(cards: string): Card[] {
  return cards.split(" ").map(parseCard);
}

function parseCard(card: string): Card {
  switch (card[0]) {
    case "s":
    case "S":
    case "4":
      return { suit: "♠", num: parseCardNum(card.slice(1)) };
    case "h":
    case "H":
    case "1":
      return { suit: "♥", num: parseCardNum(card.slice(1)) };
    case "d":
    case "D":
    case "2":
      return { suit: "♦", num: parseCardNum(card.slice(1)) };
    case "c":
    case "C":
    case "3":
      return { suit: "♣", num: parseCardNum(card.slice(1)) };
    default:
      throw new Error("Unknown suit");
  }
}

function parseCardNum(num: string): number {
  switch (num) {
    case "K":
    case "k":
      return 13;
    case "Q":
    case "q":
      return 12;
    case "J":
    case "j":
      return 11;
    case "T":
    case "t":
      return 10;
    case "A":
    case "a":
      return 1;
  }
  const ans = parseInt(num);
  if (ans > 13 || ans <= 0 || Number.isNaN(ans)) {
    throw new Error("Invalid card number");
  }

  return ans;
}
