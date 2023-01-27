<script setup lang="ts">
  import { computed, Ref, ref } from "vue";
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
    <div class="ans" :style="{ opacity: ans ? 1 : 0 }">
      <p>Win: {{ ans && (ans.win * 100).toFixed(2) }}%</p>
      <p>Tie: {{ ans && (ans.tie * 100).toFixed(2) }}%</p>
      <pre>
Hand Rate: {{ ans && JSON.stringify(ans.hand_type_rates, null, 2) }}</pre
      >
    </div>
  </div>
</template>

<style scoped>
  .input-container {
    width: 375px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    position: relative;
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
  }
</style>
