<script setup lang="ts">
const props = defineProps<{
    height: number,
    width: number
}>()
</script>

<template>
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48" id="heart" :height="height" :width="width">
    <symbol id="New_Symbol_14" viewBox="-6.5 -6.5 13 13">
      <path
        fill="#ffd4c3"
        stroke="#504b46"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-miterlimit="10"
        d="M0-6c2.2 0 4.1 1.5 4.7 3.5C6.3-2.5 6.4 0 5 0v1c0 2.8-2.2 5-5 5s-5-2.2-5-5V0c-1.4 0-1.3-2.5.2-2.5C-4.1-4.5-2.2-6 0-6z"
      ></path>
      <circle cx="-1.6" cy="-.1" r=".1" fill="#ffc258"></circle>
      <path
        fill="#4f4b45"
        d="M-1.6.5c-.3 0-.6-.3-.6-.6s.2-.7.6-.7c.3 0 .6.3.6.7s-.3.6-.6.6z"
      ></path>
      <circle cx="1.6" cy="-.1" r=".1" fill="#ffc258"></circle>
      <path
        fill="#4f4b45"
        d="M1.6.5C1.3.5 1 .2 1-.1s.3-.6.6-.6.6.3.6.6-.2.6-.6.6z"
      ></path>
      <circle cx="-3" cy="-1.5" r=".5" fill="#fabfa5"></circle>
      <circle cx="3" cy="-1.5" r=".5" fill="#fabfa5"></circle>
      <path
        fill="none"
        stroke="#504b46"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-miterlimit="10"
        d="M-1.2-3c.8-.5 1.7-.5 2.5 0"
      ></path>
    </symbol>
    <g id="Icons">
      <g id="XMLID_739_">
        <ellipse
          id="XMLID_2210_"
          cx="24"
          cy="45.4"
          fill="#45413c"
          opacity=".15"
          rx="11.5"
          ry="1.5"
        ></ellipse>
        <g id="XMLID_2206_">
          <path
            id="XMLID_2209_"
            fill="#ff6242"
            d="M24 11.1c0-5.5 4.5-10 10-10 7 0 10 7 10 12.5 0 14.6-13.8 23.9-18.5 26.7-.9.6-2.1.6-3 0C17.8 37.5 4 28.1 4 13.6 4 8 7 1.1 14 1.1c5.5 0 10 4.4 10 10z"
          ></path>
          <path
            id="XMLID_2208_"
            fill="#ff866e"
            d="M13.5 6.1c4.3 0 8 2.6 9.6 6.2.3.8 1.5.8 1.8 0 1.6-3.7 5.3-6.2 9.6-6.2 5 0 8.1 3.4 9.5 7.4C44 8 41 1.1 34 1.1c-5.5 0-10 4.5-10 10 0-5.5-4.5-10-10-10-7 0-10 6.8-10 12.3 1.4-3.9 4.5-7.3 9.5-7.3z"
          ></path>
          <path
            id="XMLID_2207_"
            fill="none"
            stroke="#45413c"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-miterlimit="10"
            d="M24 11.1c0-5.5 4.5-10 10-10 7 0 10 7 10 12.5 0 14.6-13.8 23.9-18.5 26.7-.9.6-2.1.6-3 0C17.8 37.5 4 28.1 4 13.6 4 8 7 1.1 14 1.1c5.5 0 10 4.4 10 10z"
          ></path>
        </g>
      </g>
    </g>
  </svg>
</template>

<style scoped>
svg {
    transition: width 0.35s, height 0.35s;
}
</style>