<script type="js">
    import { FilteredImage } from "wasm-swissknife";
    import { memory } from "wasm-swissknife/wasm_swissknife_bg.wasm";
    import { onMount, afterUpdate } from 'svelte';

    let video;
    let canvas;
    let animationId;
    let pre;

    onMount(async () => {
        video = document.querySelector("#videoElement");
        canvas = document.querySelector("#canvasElement");
        pre = document.querySelector("#pre");

        if (navigator.mediaDevices.getUserMedia) {
            try {
                const stream = await navigator.mediaDevices.getUserMedia({ video: true });
                video.srcObject = stream;
                video.play();
                animationId = requestAnimationFrame(drawVideoToCanvas);
            } catch (err) {
                console.log("Something went wrong!", err);
            }
        }
    });

    afterUpdate(() => {
        if (animationId) {
            cancelAnimationFrame(animationId);
        }
        animationId = requestAnimationFrame(drawVideoToCanvas);
    });

    function drawVideoToCanvas() {
        const ctx = canvas.getContext('2d', { willReadFrequently: true });
        ctx.drawImage(video, 0, 0);
        animationId = requestAnimationFrame(drawVideoToCanvas);

        const filteredImage = FilteredImage.new(640, 480);
        let imgData = ctx.getImageData(0, 0, 640, 480);
        const inputImageData = new Uint8Array(imgData.data.buffer);
        filteredImage.art(3, inputImageData)
        const chars_size_ptr = filteredImage.chars_size();
        const chars_size = new Uint32Array(memory.buffer, chars_size_ptr, 2);

        const x = chars_size[0]
        const y = chars_size[1]

        const chars_ptr = filteredImage.chars();
        const chars = new Uint8Array(memory.buffer, chars_ptr, filteredImage.chars_length());
        
        let s = ""
        for (let i = 0; i < y; i++) {
            for (let j = 0; j < x; j++) {
                const index = i * x + j
                const c = chars[index]
                s += String.fromCharCode(c)
            }       
        }
        pre.innerHTML = s
    }

</script>

<div>
    <video id="videoElement" autoplay playsinline hidden>
        <track kind="captions">
    </video>
    <canvas id="canvasElement" width="640" height="480" hidden/>
    <pre id="pre"></pre>
</div>