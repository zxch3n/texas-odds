import { describe, expect, it } from "vitest";
import { Odds, Stage } from "texas-odds";

describe("Odds Calculate", () => {
  it("case 5 cards", () => {
    const stage = new Stage(4, "hA", "hK", "sA sK sQ");
    const odds = stage.odds();
    console.dir(odds);
  });

  it("case 2 cards", () => {
    const stage = new Stage(4, "hA", "hK", "");
    const odds = stage.odds();
    console.dir(odds);
  });
});
