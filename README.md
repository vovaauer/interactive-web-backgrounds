# Interactive Web Backgrounds

A collection of beautiful, performant, and interactive web backgrounds built with Rust and WebAssembly.

**Live Demo:** [https://vovaauer.github.io/interactive-web-backgrounds/](https://vovaauer.github.io/interactive-web-backgrounds/)

---

## `aquarium`

A serene, interactive aquarium simulation featuring a procedural castle, crabs, bubbles, and intelligent fish AI that gracefully avoid boundaries and frenzy for food. Left click to spawn food and right click to spawn a fish.

### How to Use

This library is designed to be incredibly simple to add to any website.

1.  **Add a `<canvas>`** element to your HTML where you want the background to appear. Give it a unique ID.

    ```html
    <canvas id="my-background"></canvas>
    ```

2.  **Add this `<script>`** tag right before your closing `</body>` tag.

    ```html
    <script type="module">
      // Import the loader from the jsDelivr CDN, which serves it directly from GitHub.
      // Make sure to replace 'vovaauer' with your actual username.
      import 'https://cdn.jsdelivr.net/gh/vovaauer/interactive-web-backgrounds@main/loader.js';
      
      // Tell the aquarium which canvas to take over.
      window.startAquarium({
        target: '#my-background'
      });
    </script>
    ```

3.  **Style your canvas** with CSS to make it fill the desired space. For a full-page background, you can use:
    ```css
    #my-background {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        z-index: -1; /* Place it behind other content */
    }
    ```

That's it!