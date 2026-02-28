// 背景动画工具
class BackgroundAnimation {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.canvas = null;
        this.ctx = null;
        this.points = [];
        this.mouseX = 0;
        this.mouseY = 0;
        this.animationId = null;
        this.init();
    }
    
    init() {
        if (!this.container) return;
        
        this.createCanvas();
        this.createPoints();
        this.bindEvents();
        this.startAnimation();
    }
    
    createCanvas() {
        this.canvas = document.createElement('canvas');
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
        this.canvas.style.position = 'absolute';
        this.canvas.style.top = '0';
        this.canvas.style.left = '0';
        this.canvas.style.width = '100%';
        this.canvas.style.height = '100%';
        this.canvas.style.pointerEvents = 'none';
        
        this.container.appendChild(this.canvas);
        this.ctx = this.canvas.getContext('2d');
    }
    
    createPoints() {
        // 创建点阵
        const pointCount = Math.min(150, Math.floor((window.innerWidth * window.innerHeight) / 10000));
        
        for (let i = 0; i < pointCount; i++) {
            this.points.push({
                x: Math.random() * this.canvas.width,
                y: Math.random() * this.canvas.height,
                size: Math.random() * 2 + 0.5,
                speedX: (Math.random() - 0.5) * 0.5,
                speedY: (Math.random() - 0.5) * 0.5,
                color: this.getRandomColor(),
                connectionDistance: 80 + Math.random() * 70
            });
        }
    }
    
    getRandomColor() {
        const colors = [
            '#3b82f6', // 蓝色
            '#6366f1', // 靛蓝色
            '#8b5cf6', // 紫色
            '#a855f7', // 紫红色
            '#ec4899'  // 粉红色
        ];
        return colors[Math.floor(Math.random() * colors.length)];
    }
    
    bindEvents() {
        // 鼠标移动监听
        document.addEventListener('mousemove', (e) => {
            const rect = this.canvas.getBoundingClientRect();
            this.mouseX = e.clientX - rect.left;
            this.mouseY = e.clientY - rect.top;
        });
        
        // 窗口大小调整
        window.addEventListener('resize', () => {
            this.handleResize();
        });
        
        // 页面可见性变化
        document.addEventListener('visibilitychange', () => {
            if (document.hidden) {
                this.stopAnimation();
            } else {
                this.startAnimation();
            }
        });
    }
    
    handleResize() {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
        
        // 重新调整点的位置
        this.points.forEach(point => {
            point.x = (point.x / this.canvas.width) * window.innerWidth;
            point.y = (point.y / this.canvas.height) * window.innerHeight;
        });
    }
    
    startAnimation() {
        if (this.animationId) return;
        
        const animate = () => {
            this.updatePoints();
            this.draw();
            this.animationId = requestAnimationFrame(animate);
        };
        
        animate();
    }
    
    stopAnimation() {
        if (this.animationId) {
            cancelAnimationFrame(this.animationId);
            this.animationId = null;
        }
    }
    
    updatePoints() {
        for (let i = 0; i < this.points.length; i++) {
            const point = this.points[i];
            
            // 更新点的位置
            point.x += point.speedX;
            point.y += point.speedY;
            
            // 边界检查
            if (point.x < 0 || point.x > this.canvas.width) point.speedX *= -1;
            if (point.y < 0 || point.y > this.canvas.height) point.speedY *= -1;
            
            // 确保点在边界内
            point.x = Math.max(0, Math.min(this.canvas.width, point.x));
            point.y = Math.max(0, Math.min(this.canvas.height, point.y));
            
            // 鼠标交互
            const mouseDistance = Math.sqrt(
                Math.pow(point.x - this.mouseX, 2) + 
                Math.pow(point.y - this.mouseY, 2)
            );
            
            if (mouseDistance < 150) {
                const angle = Math.atan2(this.mouseY - point.y, this.mouseX - point.x);
                const force = (150 - mouseDistance) / 150 * 0.5;
                point.x += Math.cos(angle) * force;
                point.y += Math.sin(angle) * force;
            }
        }
    }
    
    draw() {
        // 清除画布
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        
        // 绘制连接线
        this.ctx.strokeStyle = 'rgba(59, 130, 246, 0.1)';
        this.ctx.lineWidth = 0.5;
        
        for (let i = 0; i < this.points.length; i++) {
            const pointA = this.points[i];
            
            for (let j = i + 1; j < this.points.length; j++) {
                const pointB = this.points[j];
                const distance = Math.sqrt(
                    Math.pow(pointA.x - pointB.x, 2) + 
                    Math.pow(pointA.y - pointB.y, 2)
                );
                
                if (distance < Math.min(pointA.connectionDistance, pointB.connectionDistance)) {
                    // 根据距离调整透明度
                    const opacity = 1 - (distance / Math.min(pointA.connectionDistance, pointB.connectionDistance));
                    
                    // 根据点颜色创建渐变
                    const gradient = this.ctx.createLinearGradient(
                        pointA.x, pointA.y,
                        pointB.x, pointB.y
                    );
                    gradient.addColorStop(0, pointA.color.replace(')', `, ${opacity * 0.3})`).replace('rgb', 'rgba'));
                    gradient.addColorStop(1, pointB.color.replace(')', `, ${opacity * 0.3})`).replace('rgb', 'rgba'));
                    
                    this.ctx.strokeStyle = gradient;
                    
                    this.ctx.beginPath();
                    this.ctx.moveTo(pointA.x, pointA.y);
                    this.ctx.lineTo(pointB.x, pointB.y);
                    this.ctx.stroke();
                }
            }
        }
        
        // 绘制点
        for (let i = 0; i < this.points.length; i++) {
            const point = this.points[i];
            
            // 创建径向渐变
            const gradient = this.ctx.createRadialGradient(
                point.x, point.y, 0,
                point.x, point.y, point.size * 2
            );
            gradient.addColorStop(0, point.color.replace(')', ', 0.8)').replace('rgb', 'rgba'));
            gradient.addColorStop(1, point.color.replace(')', ', 0)').replace('rgb', 'rgba'));
            
            this.ctx.fillStyle = gradient;
            
            this.ctx.beginPath();
            this.ctx.arc(point.x, point.y, point.size * 2, 0, Math.PI * 2);
            this.ctx.fill();
            
            // 绘制中心点
            this.ctx.fillStyle = point.color;
            this.ctx.beginPath();
            this.ctx.arc(point.x, point.y, point.size / 2, 0, Math.PI * 2);
            this.ctx.fill();
        }
        
        // 绘制鼠标周围的光晕
        if (this.mouseX > 0 && this.mouseY > 0) {
            const gradient = this.ctx.createRadialGradient(
                this.mouseX, this.mouseY, 0,
                this.mouseX, this.mouseY, 100
            );
            gradient.addColorStop(0, 'rgba(59, 130, 246, 0.1)');
            gradient.addColorStop(1, 'rgba(59, 130, 246, 0)');
            
            this.ctx.fillStyle = gradient;
            this.ctx.beginPath();
            this.ctx.arc(this.mouseX, this.mouseY, 100, 0, Math.PI * 2);
            this.ctx.fill();
        }
    }
    
    // 添加新点
    addPoint(x, y) {
        this.points.push({
            x: x || Math.random() * this.canvas.width,
            y: y || Math.random() * this.canvas.height,
            size: Math.random() * 2 + 0.5,
            speedX: (Math.random() - 0.5) * 0.5,
            speedY: (Math.random() - 0.5) * 0.5,
            color: this.getRandomColor(),
            connectionDistance: 80 + Math.random() * 70
        });
    }
    
    // 移除点
    removePoint(index) {
        if (index >= 0 && index < this.points.length) {
            this.points.splice(index, 1);
        }
    }
    
    // 清空所有点
    clearPoints() {
        this.points = [];
    }
    
    // 重新生成点
    regeneratePoints(count) {
        this.clearPoints();
        for (let i = 0; i < count; i++) {
            this.addPoint();
        }
    }
    
    // 获取当前点数量
    getPointCount() {
        return this.points.length;
    }
    
    // 设置动画速度
    setSpeed(multiplier) {
        this.points.forEach(point => {
            point.speedX *= multiplier;
            point.speedY *= multiplier;
        });
    }
    
    // 销毁动画
    destroy() {
        this.stopAnimation();
        if (this.canvas && this.canvas.parentNode) {
            this.canvas.parentNode.removeChild(this.canvas);
        }
        
        // 移除事件监听器
        document.removeEventListener('mousemove', this.handleMouseMove);
        window.removeEventListener('resize', this.handleResize);
        document.removeEventListener('visibilitychange', this.handleVisibilityChange);
    }
}

// 初始化背景动画
document.addEventListener('DOMContentLoaded', () => {
    if (document.getElementById('animationBackground')) {
        window.backgroundAnimation = new BackgroundAnimation('animationBackground');
    }
});