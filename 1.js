(window.webpackJsonp=window.webpackJsonp||[]).push([[1],[,function(t,i,h){"use strict";h.r(i),h.d(i,"Renderer",(function(){return c}));class c{constructor(t){this.config=t,this.circles=[];for(let i=0;i<t.numberOfCircles;++i)this.circles.push({x:Math.random()*t.width,y:Math.random()*t.height,dx:2*Math.random()-1,dy:2*Math.random()-1,h:Math.floor(360*Math.random())})}update(){for(const t of this.circles)t.x+=t.dx,t.y+=t.dy,(t.x<0||this.config.width<t.x)&&(t.dx=-t.dx),(t.y<0||this.config.height<t.y)&&(t.dy=-t.dy)}render(t){t.clearRect(0,0,this.config.width,this.config.height);for(const i of this.circles)t.fillStyle=`hsla(${i.h},100%,50%,0.5)`,t.beginPath(),t.arc(i.x,i.y,this.config.radius,0,2*Math.PI),t.fill()}}}]]);