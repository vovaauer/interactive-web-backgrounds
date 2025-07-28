// loader.js - The user-facing API

// This function will be the public API
async function startAquarium(config) {
    // Find the canvas element the user wants to use
    const canvas = document.querySelector(config.target);
    if (!canvas) {
        console.error(`Aquarium background error: Target element "${config.target}" not found.`);
        return;
    }

    // Give the canvas a unique ID for Rust to find.
    // We use the config.target to make it unique if multiple aquariums are on a page.
    const canvasId = 'interactive-background-canvas-' + config.target.replace('#', '').replace('.', '');
    canvas.id = canvasId;

    try {
        // Dynamically import the Rust code from the 'pkg' directory.
        // This path assumes 'loader.js' and 'pkg/' are in the same directory.
        const { default: init, Aquarium } = await import('./pkg/aquarium.js');

        // Load the .wasm file.
        await init('./pkg/aquarium_bg.wasm');

        // Create an instance of our Rust simulation, telling it which canvas to use.
        const aquarium = new Aquarium(canvasId);

        // --- Event Listeners ---
        canvas.addEventListener('click', (event) => {
            aquarium.add_food(event.offsetX, event.offsetY);
        });

        canvas.addEventListener('contextmenu', (event) => {
            event.preventDefault();
            aquarium.add_fish(event.offsetX, event.offsetY);
        });

        // --- Animation Loop ---
        function animate() {
            aquarium.tick();
            requestAnimationFrame(animate);
        }
        
        // Start the loop
        requestAnimationFrame(animate);

    } catch (e) {
        console.error("Error loading aquarium background:", e);
    }
}

// Expose the function globally so it can be called from a simple <script> tag.
window.startAquarium = startAquarium;