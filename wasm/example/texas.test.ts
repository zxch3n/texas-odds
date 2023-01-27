import { describe, expect, it } from "vitest";
import { Odds, Stage } from "texas-odds";

describe("Odds Calculate", () => {
  it("case 0", () => {
    const stage = new Stage(4, "hA", "hK", "sA sK sQ");
    const odds = stage.odds();
    console.log(odds.tie);
    console.log(odds.win);
    console.log(odds.hand_type_rates());
  });
});
