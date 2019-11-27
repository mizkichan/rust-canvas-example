export class Renderer {
  constructor(config) {
    this.config = config;
    this.circles = [];
    for (let i = 0; i < config.numberOfCircles; ++i) {
      this.circles.push({
        x: Math.random() * config.width,
        y: Math.random() * config.height,
        dx: Math.random() * 2 - 1,
        dy: Math.random() * 2 - 1,
        h: Math.floor(Math.random() * 360)
      });
    }
  }

  update() {
    for (const circle of this.circles) {
      circle.x += circle.dx;
      circle.y += circle.dy;
      if (circle.x < 0 || this.config.width < circle.x) {
        circle.dx = -circle.dx;
      }
      if (circle.y < 0 || this.config.height < circle.y) {
        circle.dy = -circle.dy;
      }
    }
  }

  render(context) {
    context.clearRect(0, 0, this.config.width, this.config.height);
    for (const circle of this.circles) {
      context.fillStyle = `hsla(${circle.h},100%,50%,0.5)`;
      context.beginPath();
      context.arc(circle.x, circle.y, this.config.radius, 0, Math.PI * 2);
      context.fill();
    }
  }
}

// vim: set ts=2 sw=2 et:
