<script lang="ts">
  let canvasEl: HTMLCanvasElement | undefined = $state();

  interface Particle {
    x: number;
    y: number;
    size: number;
    speedX: number;
    speedY: number;
    opacity: number;
    opacityDir: number;
    color: string;
    shape: 'circle' | 'star' | 'diamond';
  }

  const COLORS = ['#ff2d95', '#00f0ff', '#c77dff', '#ffb7c5', '#7b68ee'];
  const PARTICLE_COUNT = 28;

  function createParticle(width: number, height: number): Particle {
    const shapes: Particle['shape'][] = ['circle', 'star', 'diamond'];
    return {
      x: Math.random() * width,
      y: Math.random() * height,
      size: Math.random() * 3 + 1,
      speedX: (Math.random() - 0.5) * 0.3,
      speedY: -(Math.random() * 0.4 + 0.1),
      opacity: Math.random() * 0.6 + 0.1,
      opacityDir: Math.random() > 0.5 ? 1 : -1,
      color: COLORS[Math.floor(Math.random() * COLORS.length)],
      shape: shapes[Math.floor(Math.random() * shapes.length)],
    };
  }

  function drawStar(ctx: CanvasRenderingContext2D, cx: number, cy: number, r: number) {
    const spikes = 4;
    const outerR = r;
    const innerR = r * 0.4;
    ctx.beginPath();
    for (let i = 0; i < spikes * 2; i++) {
      const radius = i % 2 === 0 ? outerR : innerR;
      const angle = (Math.PI * i) / spikes - Math.PI / 2;
      const x = cx + Math.cos(angle) * radius;
      const y = cy + Math.sin(angle) * radius;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.closePath();
    ctx.fill();
  }

  function drawDiamond(ctx: CanvasRenderingContext2D, cx: number, cy: number, r: number) {
    ctx.beginPath();
    ctx.moveTo(cx, cy - r);
    ctx.lineTo(cx + r * 0.6, cy);
    ctx.lineTo(cx, cy + r);
    ctx.lineTo(cx - r * 0.6, cy);
    ctx.closePath();
    ctx.fill();
  }

  $effect(() => {
    const canvas = canvasEl;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const width = canvas.offsetWidth;
    const height = canvas.offsetHeight;
    canvas.width = width * 2;
    canvas.height = height * 2;
    ctx.scale(2, 2);

    const particles: Particle[] = Array.from({ length: PARTICLE_COUNT }, () =>
      createParticle(width, height)
    );

    let animId: number;

    function animate() {
      ctx!.clearRect(0, 0, width, height);

      for (const p of particles) {
        p.x += p.speedX;
        p.y += p.speedY;
        p.opacity += p.opacityDir * 0.004;

        if (p.opacity >= 0.7) p.opacityDir = -1;
        if (p.opacity <= 0.05) p.opacityDir = 1;

        if (p.y < -10) {
          p.y = height + 5;
          p.x = Math.random() * width;
        }
        if (p.x < -10) p.x = width + 5;
        if (p.x > width + 10) p.x = -5;

        ctx!.globalAlpha = p.opacity;
        ctx!.fillStyle = p.color;

        // glow
        ctx!.shadowBlur = p.size * 3;
        ctx!.shadowColor = p.color;

        if (p.shape === 'circle') {
          ctx!.beginPath();
          ctx!.arc(p.x, p.y, p.size, 0, Math.PI * 2);
          ctx!.fill();
        } else if (p.shape === 'star') {
          drawStar(ctx!, p.x, p.y, p.size * 1.5);
        } else {
          drawDiamond(ctx!, p.x, p.y, p.size * 1.5);
        }

        ctx!.shadowBlur = 0;
      }

      ctx!.globalAlpha = 1;
      animId = requestAnimationFrame(animate);
    }

    animate();

    return () => cancelAnimationFrame(animId);
  });
</script>

<canvas bind:this={canvasEl} class="particles-canvas"></canvas>

<style>
  .particles-canvas {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 0;
  }
</style>
