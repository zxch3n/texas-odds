<script setup lang="ts">
  import { computed, ref } from "vue";
  import { parseCards } from "./parseCard";
  import { calc } from "./calc";

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
    try {
      return parseCards(commCardsText.value);
    } catch (e) {
      return undefined;
    }
  });
  const btnValid = computed(() => {
    return (
      holeCards.value &&
      communityCards.value &&
      holeCards.value.length === 2 &&
      (communityCards.value.length == 0 ||
        (communityCards.value.length >= 3 && communityCards.value.length <= 5))
    );
  });
  const ans = ref(undefined);
  function calculate() {
    calc(players.value, holeCardsText.value, commCardsText.value);
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
    margin-bottom: 8px;
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
    height: 200px;
  }
</style>
