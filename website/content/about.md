+++
title = "About Graphite"

[extra]
css = "/about.css"
+++

<section class="section-row">
<div class="section">

# About Graphite

<!-- TODO -->

## The project

The idea for Graphite began with a desire to create artwork and edit photos using free software that felt user-friendly and truly modern. Over time, that dream evolved to redefine what "modern" meant for the world of 2D graphics editing. By borrowing concepts popular in 3D software, what could a procedural, non-destructive design tool look like if nothing was too ambitious? Answering that question took years of design exploration, leading to an open source developer community of savvy volunteers striving to turn this formidable dream into a reality.

</div>
</section>

<section class="section-row">

<div class="diptych">

<div class="section">

## Mission

Creativity should be a life-long journey without limits. From students to studios, creative software should be accessible and approachable ...

- To ambitiously build the ultimate 2D digital content creation application for most users and use cases.
- To thoughtfully employ user-friendly design in every inch of the experience, sacrificing neither form nor function.
- To aggressively innovate by pairing the latest advancements in technology with zero-compromise engineering.
- To freely provide professional software that's accessible and useful to creatives ranging from students to studios.

<script>
(async () => {
	const response = await fetch("https://api.github.com/repos/graphiteeditor/graphite?per_page=1");
	const json = await response.json();
	const stars = parseInt(json.stargazers_count);
	if (!stars) return;

	document.querySelector("[data-github-stars]").innerText = `${Math.round(stars / 100) / 10}k`;
})();

(async () => {
	const response = await fetch("https://api.github.com/repos/graphiteeditor/graphite/commits?per_page=1");
	const link = [...response.headers].find(([header, _]) => header === "link")[1];
	if (!link) return;
	// With one page per commit, the last past number is the commit count
	const commits = parseInt(link.match(/page=(\d+)>; rel="last"/)[1]);
	if (!commits) return;

	document.querySelector("[data-code-commits]").innerText = commits;
})();
</script>

</div>
<div class="section">

## In numbers

- Development began: February 2021
- GitHub stars: <span data-github-stars></span>
- Code commits: <span data-code-commits></span>

Graphite combines the best ideas from multiple categories of digital content creation software to form a design for the ultimate general-purpose 2D graphics editor. It is influenced by the central editing experience of traditional layer-based raster and vector apps. It takes inspiration from the non-destructive workflows of VFX compositing programs used in Hollywood. And it borrows the creative superpowers of procedural asset creation applications in the 3D industry.

</div>

</div>

</section>

<section id="opener-message" class="section-row">
<div class="section">

## Professional 2D content creation for everyone

With great power comes great accessibility. Graphite is built on the belief that the best creative tools can be powerful and within reach of all.

Graphite is designed with a friendly and intuitive interface where a delightful user experience is of first-class importance. It is available for free under an open source [license](/license) and usable [instantly through a web browser](https://editor.graphite.rs) or an upcoming native client on Windows, Mac, and Linux.

The accessible design of Graphite does not sacrifice versatility for simplicity. The node-based workflow opens doors to an ecosystem of powerful capabilities catering to the casual and professional user alike, encompassing a wide set of use cases at every skill level.

</div>
<div class="graphic">
	<img src="https://static.graphite.rs/content/index/brush__2.svg" alt="" />
</div>
</section>

<section id="upcoming-tech" class="feature-box">
<div class="box">

<h1 class="box-header">Upcoming Tech: How it works</h1>

---

<!-- Graphite's concept is unique among graphics editors and requires some explanation. Here's a glimpse at that secret sauce. -->

<div class="diptych">

<div class="section">

## Layers & nodes: hybrid compositing

Graphite combines the best ideas from multiple categories of digital content creation software to form a design for the ultimate general-purpose 2D graphics editor. It is influenced by the central editing experience of traditional layer-based raster and vector apps. It takes inspiration from the non-destructive workflows of VFX compositing programs used in Hollywood. And it borrows the creative superpowers of procedural asset creation applications in the 3D industry.

Classic layer-based image editing is easy to understand and its collapsable folders help artists stay organized. A variety of interactive viewport tools make it easy to manipulate the layers by drawing directly onto the canvas. On the other hand, node-based editing is like artist-friendly programming. It works by describing manipulations as steps in a flowchart, which is vastly more powerful but comes with added complexity.

The hybrid workflow of Graphite offers a classic tool-centric, layer-based editing experience built around a procedural, node-based compositor. Users can ignore the node graph, use it exclusively, or switch back and forth with the press of a button while creating content. Interacting with the canvas using tools will manipulate the nodes behind the scenes. And the layer panel and node graph provide two equivalent, interchangeable views of the same document structure.

</div>
<div class="section">

## Raster & vector: sharp at all sizes

Digital 2D art commonly takes two forms. Raster artwork is made out of pixels which means it can look like anything imaginable, but it becomes blurry or pixelated from upscaling to a higher resolution. Vector artwork is made out of curved shapes which is perfect for some art styles but limiting to others. The magic of vector is that its mathematically-described curves can be enlarged to any size and remain crisp.

Other apps usually focus on just raster or vector, forcing artists to buy and learn both products. Mixing art styles requires shuttling content back and forth between programs. And since picking a raster document resolution is a one-time deal, artists may choose to start really big, resulting in sluggish editing performance and multi-gigabyte documents.

Graphite reinvents raster rendering so it stays sharp at any scale. Artwork is treated as data, not pixels, and is always drawn at the current view resolution. Zoom the viewport and export images at any size— the document's paint brushes, masks, filters, and effects will all be rendered at the native resolution.

Marrying vector and raster under one roof enables both art forms to complement each other in a holistic content creation workflow.

</div>

</div>

</div>
</section>

<section class="section-row">
<div class="section">

## Powered by Graphene

Graphene is the node graph engine that powers Graphite's compositor and procedural graphics pipeline. Graphite comes with nodes and data formats useful for graphics editing, while Graphene is more general-purpose. It's a visual scripting environment built upon the high-performance Rust programming language. The Graphene runtime distributes work across the CPU, GPU, and network/cloud hardware, optimizing for interactive frame rates.

Rust programmers may find the following technical details to be of interest. Graphene node graphs are programs built out of reusable Rust functions using Graphite as a visual "code" editor. New nodes and data types can be implemented by writing custom Rust code with a built-in text editor. `no_std` code also gets compiled to GPU compute shaders using [`rust-gpu`](https://github.com/EmbarkStudios/rust-gpu). Each node is independently pre-compiled by `rustc` into portable WASM binaries and linked at runtime. Groups of nodes may be compiled into one unit of execution, utilizing Rust's zero-cost abstractions and optimizations to run with less overhead. And whole node graphs can be compiled into standalone executables for use outside Graphite.

</div>
</section>

<section id="powered-by-rust" class="section-row left">

<div class="graphic">
	<img src="https://static.graphite.rs/content/index/rust.svg" alt="" />
</div>
<div class="section">

<!-- ## Proudly written in Rust -->
## Built for the future, powered by Rust

Always on the bleeding edge and built to last— Graphite is written on a robust foundation with Rust, a modern programming language optimized for creating fast, reliable, future-proof software.

The underlying node graph engine that computes and renders Graphite documents is called Graphene. The Graphene engine is an extension of the Rust language, acting as a system for chaining together modular functions into useful pipelines with GPU and parallel computation. Artists can harness these powerful capabilities directly in the Graphite editor without touching code. Technical artists and programmers can write reusable Rust functions to extend the capabilities of Graphite and create new nodes to share with the community.

</div>

</section>

<section id="get-involved" class="section-row">
<div class="section">

## Get involved

The Graphite project could not exist without the community. Building its ambitious and versatile featureset will require contributions from developers, designers, technical experts, creative professionals, and eagle-eyed bug hunters. Help build the future of digital art!

<a href="/contribute" class="link arrow">Contribute</a>

</div>
<div class="graphic">
	<img src="https://static.graphite.rs/content/index/volunteer.svg" alt="" />
</div>
</section>

<section class="section-row">
<div class="section">

## Development roadmap

Short-to-medium-term feature development is tracked at a granular level in the [Task Board](https://github.com/GraphiteEditor/Graphite/projects/1) on GitHub. Graphite uses a continuous release cycle without version numbers where changes can be tracked by [commit hash](https://github.com/GraphiteEditor/Graphite/commits/master). The stable release at [editor.graphite.rs](https://editor.graphite.rs) deploys a [recent commit](https://github.com/GraphiteEditor/Graphite/releases/tag/latest-stable) from the past several weeks, while [dev.graphite.rs](https://dev.graphite.rs) hosts the latest commit.

<!-- and [monthly sprints](https://github.com/GraphiteEditor/Graphite/milestones). -->
<!-- Development broke ground in February 2021. -->
<!-- TODO -->

<div class="roadmap">
	<div class="informational-group features">
		<div class="informational complete heading">
			<h3>— Pre-Alpha —</h3>
		</div>
		<div class="informational complete">
			<img class="atlas" style="--atlas-index: 0" src="icon-atlas-roadmap.png" alt="" />
			<span>First year of development (details omitted)</span>
		</div>
		<div class="informational ongoing heading">
			<h3>— Alpha Milestone 1 —</h3>
		</div>
		<div class="informational complete">
			<img class="atlas" style="--atlas-index: 1" src="icon-atlas-roadmap.png" alt="" />
			<span>Second year of development (details omitted)</span>
		</div>
		<div class="informational ongoing">
			<img class="atlas" style="--atlas-index: 4" src="icon-atlas-roadmap.png" alt="" />
			<span>Brush tool</span>
		</div>
		<div class="informational ongoing">
			<img class="atlas" style="--atlas-index: 11" src="icon-atlas-roadmap.png" alt="" />
			<span>WebGPU in supported browsers</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 2" src="icon-atlas-roadmap.png" alt="" />
			<span>Vertical compositing of nodes</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 3" src="icon-atlas-roadmap.png" alt="" />
			<span>Node-based layer tree</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 5" src="icon-atlas-roadmap.png" alt="" />
			<span>Graph-based documents</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 6" src="icon-atlas-roadmap.png" alt="" />
			<span>Self-updating desktop app</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 7" src="icon-atlas-roadmap.png" alt="" />
			<span>Custom subgraph document nodes</span>
		</div>
		<div class="informational heading">
			<h3>— Alpha Milestone 2 —</h3>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 8" src="icon-atlas-roadmap.png" alt="" />
			<span>Graph data attributes</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 9" src="icon-atlas-roadmap.png" alt="" />
			<span>Spreadsheet-based vector data</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 10" src="icon-atlas-roadmap.png" alt="" />
			<span>Editable SVG import</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 12" src="icon-atlas-roadmap.png" alt="" />
			<span>Rust-based vector renderer</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 22" src="icon-atlas-roadmap.png" alt="" />
			<span>Select mode and masking</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 13" src="icon-atlas-roadmap.png" alt="" />
			<span>New viewport overlays system</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 14" src="icon-atlas-roadmap.png" alt="" />
			<span>Resolution-agnostic raster rendering</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 15" src="icon-atlas-roadmap.png" alt="" />
			<span>Powerful snapping and grid system</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 16" src="icon-atlas-roadmap.png" alt="" />
			<span>Remote compile/render server</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 17" src="icon-atlas-roadmap.png" alt="" />
			<span>Code editor for custom nodes</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 19" src="icon-atlas-roadmap.png" alt="" />
			<span>Document history system</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 18" src="icon-atlas-roadmap.png" alt="" />
			<span>Stable document format</span>
		</div>
		<div class="informational heading">
			<h3>— Alpha Milestone 3 —</h3>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 20" src="icon-atlas-roadmap.png" alt="" />
			<span>RAW photo import and processing</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 21" src="icon-atlas-roadmap.png" alt="" />
			<span>Procedural paint brush styling</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 23" src="icon-atlas-roadmap.png" alt="" />
			<span>Frozen history references</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 24" src="icon-atlas-roadmap.png" alt="" />
			<span>Internationalization and accessability</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 25" src="icon-atlas-roadmap.png" alt="" />
			<span>Reconfigurable workspace panels</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 26" src="icon-atlas-roadmap.png" alt="" />
			<span>Liquify and non-affine rendering</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 27" src="icon-atlas-roadmap.png" alt="" />
			<span>Interactive graph auto-layout</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 28" src="icon-atlas-roadmap.png" alt="" />
			<span>Automation and batch processing</span>
		</div>
		<div class="informational heading">
			<h3>— Beta —</h3>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 29" src="icon-atlas-roadmap.png" alt="" />
			<span>Guide mode</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 30" src="icon-atlas-roadmap.png" alt="" />
			<span>CAD-like constraint solver</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 31" src="icon-atlas-roadmap.png" alt="" />
			<span>Constraint models for UI layouts</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 32" src="icon-atlas-roadmap.png" alt="" />
			<span>Advanced typography and typesetting</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 33" src="icon-atlas-roadmap.png" alt="" />
			<span>PDF export</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 34" src="icon-atlas-roadmap.png" alt="" />
			<span>HDR and WCG color handling</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 35" src="icon-atlas-roadmap.png" alt="" />
			<span>Node manager and marketplace</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 36" src="icon-atlas-roadmap.png" alt="" />
			<span>Predictive graph rendering/caching</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 37" src="icon-atlas-roadmap.png" alt="" />
			<span>Distributed graph rendering</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 38" src="icon-atlas-roadmap.png" alt="" />
			<span>Cloud document storage</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 39" src="icon-atlas-roadmap.png" alt="" />
			<span>Multiplayer collaborative editing</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 40" src="icon-atlas-roadmap.png" alt="" />
			<span>Offline edit resolution with CRDTs</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 41" src="icon-atlas-roadmap.png" alt="" />
			<span>Native UI rewrite from HTML frontend</span>
		</div>
		<div class="informational heading">
			<h3>— 1.0 Release —</h3>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 42" src="icon-atlas-roadmap.png" alt="" />
			<span>Timeline and renderer for animation</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 43" src="icon-atlas-roadmap.png" alt="" />
			<span>Live video compositing</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 44" src="icon-atlas-roadmap.png" alt="" />
			<span>Pen and touch-only interaction</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 45" src="icon-atlas-roadmap.png" alt="" />
			<span>iPad app</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 46" src="icon-atlas-roadmap.png" alt="" />
			<span>Portable render engine</span>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 48" src="icon-atlas-roadmap.png" alt="" />
			<span>SVG animation</span>
		</div>
		<div class="informational heading">
			<h3>— Future Releases —</h3>
		</div>
		<div class="informational">
			<img class="atlas" style="--atlas-index: 49" src="icon-atlas-roadmap.png" alt="" />
			<span><em>…and that's just the beginning…</em></span>
		</div>
	</div>
</div>

</div>
</section>
