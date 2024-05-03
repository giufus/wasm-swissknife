<script>
  import { onMount } from 'svelte';
  import { transcriptStore } from '../store.js';

  onMount(() => {
    const recognition = new (window.SpeechRecognition || window.webkitSpeechRecognition)();
    recognition.interimResults = true;

    recognition.addEventListener('result', event => {
      const transcriptArray = Array.from(event.results)
        .filter(result => result.isFinal)
        .map(result => result[0])
        .map(result => result.transcript)
        .join('');
      transcriptStore.update(value => value + ' ' + transcriptArray);
    });

    recognition.addEventListener('end', recognition.start);

    recognition.start();
  });
</script>

<div>
  <p>{$transcriptStore}</p>
</div>