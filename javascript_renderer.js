class JavaScriptRenderer {
  constructor(context, width, height, radius) {
    this.context = context;
    this.width = width;
    this.height = height;
    this.radius = radius;
    this.circles = [];
  }

  setCirclesCount(n) {
    if (n < this.circles.length) {
      this.circles.splice(n);
    } else if (this.circles.length < n) {
      n -= this.circles.length;
      for (let i = 0; i < n; ++i) {
        this.circles.push(gen_circle(this.radius, this.width, this.height));
      }
    }
  }

  update() {
    this.context.clearRect(0, 0, this.width, this.height);

    for (let circle of this.circles) {
      this.context.fillStyle = `hsl(${circle.h},100%,50%,0.5)`;

      this.context.beginPath();
      this.context.arc(circle.x, circle.y, circle.r, 0, Math.PI * 2);
      this.context.fill();

      circle.x = circle.x + circle.dx;
      if (circle.x < 0) {
        circle.x = -circle.x;
        circle.dx = -circle.dx;
      } else if (this.width < circle.x) {
        circle.x = this.width * 2 - circle.x;
        circle.dx = -circle.dx;
      }

      circle.y = circle.y + circle.dy;
      if (circle.y < 0) {
        circle.y = -circle.y;
        circle.dy = -circle.dy;
      } else if (this.height < circle.y) {
        circle.y = this.height * 2 - circle.y;
        circle.dy = -circle.dy;
      }
    }
  }
}

const gen_circle = (r, width, height) => {
  const t = Math.random() * Math.PI * 2;
  return {
    r,
    x: Math.random() * width,
    y: Math.random() * height,
    dx: Math.cos(t),
    dy: Math.sin(t),
    h: Math.random() * 360
  };
};

export default JavaScriptRenderer;

// vim: set ts=2 sw=2 et:
