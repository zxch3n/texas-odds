<script setup lang="ts">
  import GitHub from "./components/GitHub.vue";
  import { computed, ref } from "vue";
  import { parseCards } from "./parseCard";
  import { calc, Odds } from "./calc";

  const players = ref(2);
  const holeCardsText = ref("");
  const commCardsText = ref("");
  const holeCards = computed(() => {
    try {
      return parseCards(holeCardsText.value);
    } catch (e) {
      return undefined;
    }
  });
  const communityCards = computed(() => {
    if (commCardsText.value.length === 0) {
      return [];
    }

    try {
      return parseCards(commCardsText.value);
    } catch (e) {
      return undefined;
    }
  });
  const btnValid = computed(() => {
    return (
      holeCards.value &&
      communityCards.value != null &&
      holeCards.value.length === 2 &&
      (communityCards.value.length == 0 ||
        (communityCards.value.length >= 3 && communityCards.value.length <= 5))
    );
  });
  const ans = ref(undefined as undefined | Odds);
  function calculate() {
    const odds = calc(players.value, holeCardsText.value, commCardsText.value);
    ans.value = odds;
  }
</script>

<template>
  <div class="container">
    <div class="input-container">
      <div><span>Players</span> <input v-model="players" type="number" /></div>
      <div><span>Hole Cards</span> <input v-model="holeCardsText" /></div>
      <div>
        <span>Community Cards</span>
        <input v-model="commCardsText" />
      </div>
    </div>
    <div class="cards">
      <div>
        <span>Hole Cards</span>
        <span>{{ holeCards?.map((x) => `${x.suit}${x.num}`).join(" ") }}</span>
      </div>
      <div>
        <span>Community Cards</span>
        <span>{{
          communityCards?.map((x) => `${x.suit}${x.num}`).join(" ")
        }}</span>
      </div>
    </div>

    <button :disabled="!btnValid" @click="calculate">Calculate</button>
    <div class="ans" :hidden="!ans">
      <p>Win: {{ ans && (ans.win * 100).toFixed(2) }}%</p>
      <p>Tie: {{ ans && (ans.tie * 100).toFixed(2) }}%</p>
      <pre>
Hand Rate: {{ ans && JSON.stringify(ans.hand_type_rates, null, 2) }}</pre
      >
    </div>
    <div class="help" :hidden="!!ans">
      <p>Input format:</p>
      <pre>h2 d3 s4 c5 d13</pre>
      <pre>️♥️2, ♦️3 ♠️4, ♣️5, ♦️K</pre>
      <ul style="margin-left: -1.5em">
        <li>h = hearts 红心 ♥️</li>
        <li>d = diamonds 方块 ♦️</li>
        <li>s = spades 黑桃 ♠️</li>
        <li>c = clubs 梅花 ♣️</li>
      </ul>
    </div>
    <div class="repo">
      <a href="https://github.com/zxch3n/texas-odds" target="_blank"
        ><GitHub style="font-size: 1.3em; margin-right: 4px" />Made by zxch3n</a
      >
    </div>
  </div>
</template>

<style scoped>
  .input-container {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .input-container > div {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    margin-bottom: 12px;
    width: 100%;
  }

  .cards > div {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    margin-bottom: 8px;
    width: 100%;
  }

  .input-container > div > span {
    margin-right: 8px;
  }

  .cards {
    height: 100px;
  }

  .ans {
    text-align: left;
    height: 400px;
    max-width: 360px;
    white-space: pre-wrap;
  }

  .ans pre {
    font-size: 0.85em;
    line-height: 1.4em;
  }

  .help {
    margin-top: 40px;
    opacity: 0.9;
    color: #bbb;
  }

  .help ul {
    list-style-type: none;
  }

  .repo {
    margin-top: 40px;
    font-size: 0.9em;
    color: grey;
  }

  .repo a {
    color: grey;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .repo a:hover {
    color: #aaa;
  }
</style>
