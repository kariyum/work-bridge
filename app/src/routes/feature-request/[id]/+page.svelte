  <script lang="ts">
	import { ThumbsUp, ThumbsDown, Trash2 } from "lucide-svelte";

    let { data } = $props();
</script>

<div class="container">
    <div class="sub-container">
        <h1>#{data.featureRequest?.id} - {data.featureRequest?.title}</h1>
        <div class="status">
            TODO
        </div>
        <p>
            {data.featureRequest?.description}
        </p>
        <button class="emoji no-line-height"><ThumbsUp /></button>
        <button class="emoji no-line-height"><ThumbsDown  /></button>
        <button class="emoji no-line-height"><Trash2 /></button>
        <div>
            <h2>Comments</h2>
            <input type="text" name="comment" placeholder="write a comment" id="comment">
        </div>
    </div>
</div>


<style>
    .emoji {
        margin: 0;
        padding: 0.3rem;
        background-color: transparent;
        border: none;
    }
    .sub-container {
        max-width: 1100px;
        margin: auto;
    }
    
</style>