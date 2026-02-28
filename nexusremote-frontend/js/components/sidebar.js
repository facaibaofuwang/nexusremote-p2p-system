// 侧边栏导航组件
class Sidebar {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.currentPage = 'dashboard';
        this.init();
    }
    
    init() {
        this.render();
        this.bindEvents();
    }
    
    render() {
        this.container.innerHTML = `
            <aside class="w-64 bg-secondary border-r border-dark-lighter flex flex-col">
                <!-- Logo -->
                <div class="p-4 border-b border-dark-lighter flex items-center space-x-3">
                    <div class="w-10 h-10 rounded-lg bg-accent flex items-center justify-center">
                        <i class="fa fa-connectdevelop text-white text-xl"></i>
                    </div>
                    <div>
                        <h1 class="text-xl font-bold text-white">NexusRemote</h1>
                        <p class="text-xs text-gray-400">v3.0 Final</p>
                    </div>
                </div>

                <!-- 导航菜单 -->
                <nav class="flex-1 overflow-y-auto scrollbar-thin py-4">
                    <ul class="space-y-1 px-2">
                        <li>
                            <a href="#" data-page="dashboard" class="flex items-center space-x-3 px-4 py-3 rounded-lg ${this.currentPage === 'dashboard' ? 'bg-accent bg-opacity-20 text-white' : 'hover:bg-dark-lighter transition-colors'}">
                                <i class="fa fa-tachometer w-5 text-center"></i>
                                <span>仪表盘</span>
                            </a>
                        </li>
                        <li>
                            <a href="connect.html" data-page="connect" class="flex items-center space-x-3 px-4 py-3 rounded-lg hover:bg-dark-lighter transition-colors">
                                <i class="fa fa-desktop w-5 text-center"></i>
                                <span>远程控制</span>
                            </a>
                        </li>
                        <li>
                            <a href="#" data-page="network" class="flex items-center space-x-3 px-4 py-3 rounded-lg hover:bg-dark-lighter transition-colors">
                                <i class="fa fa-sitemap w-5 text-center"></i>
                                <span>网络节点</span>
                            </a>
                        </li>
                        <li>
                            <a href="#" data-page="tokens" class="flex items-center space-x-3 px-4 py-3 rounded-lg hover:bg-dark-lighter transition-colors">
                                <i class="fa fa-exchange w-5 text-center"></i>
                                <span>通证管理</span>
                            </a>
                        </li>
                        <li>
                            <a href="#" data-page="security" class="flex items-center space-x-3 px-4 py-3 rounded-lg hover:bg-dark-lighter transition-colors">
                                <i class="fa fa-shield w-5 text-center"></i>
                                <span>安全中心</span>
                            </a>
                        </li>
                        <li>
                            <a href="#" data-page="settings" class="flex items-center space-x-3 px-4 py-3 rounded-lg hover:bg-dark-lighter transition-colors">
                                <i class="fa fa-cogs w-5 text-center"></i>
                                <span>系统设置</span>
                            </a>
                        </li>
                    </ul>

                    <!-- 节点状态 -->
                    <div class="mt-8 px-4">
                        <h3 class="text-xs uppercase text-gray-500 font-semibold mb-2">节点状态</h3>
                        <div class="space-y-3">
                            <div class="flex justify-between items-center mb-2">
                                <span class="text-sm">在线节点</span>
                                <span class="text-sm font-medium text-success">127</span>
                            </div>
                            <div class="flex justify-between items-center mb-2">
                                <span class="text-sm">中继节点</span>
                                <span class="text-sm font-medium text-accent">42</span>
                            </div>
                            <div class="flex justify-between items-center">
                                <span class="text-sm">网络健康度</span>
                                <span class="text-sm font-medium text-success">92%</span>
                            </div>
                        </div>
                    </div>

                    <!-- 用户信息 -->
                    <div class="mt-auto p-4 border-t border-dark-lighter">
                        <div class="flex items-center space-x-3">
                            <div class="w-10 h-10 rounded-full bg-accent bg-opacity-20 flex items-center justify-center">
                                <i class="fa fa-user text-accent"></i>
                            </div>
                            <div class="flex-1">
                                <p class="text-sm font-medium text-white">管理员</p>
                                <p class="text-xs text-gray-400">nexusremote@example.com</p>
                            </div>
                            <button class="text-gray-400 hover:text-white transition-colors">
                                <i class="fa fa-sign-out"></i>
                            </button>
                        </div>
                    </div>
                </nav>
            </aside>
        `;
    }
    
    bindEvents() {
        // 绑定导航点击事件
        const navLinks = this.container.querySelectorAll('a[data-page]');
        navLinks.forEach(link => {
            link.addEventListener('click', (e) => {
                e.preventDefault();
                const page = link.getAttribute('data-page');
                this.navigateTo(page);
            });
        });
    }
    
    navigateTo(page) {
        this.currentPage = page;
        this.render();
        
        // 触发页面切换事件
        const event = new CustomEvent('pageChange', { detail: { page } });
        document.dispatchEvent(event);
    }
    
    getCurrentPage() {
        return this.currentPage;
    }
    
    updateNodeStatus(onlineNodes, relayNodes, health) {
        // 更新节点状态信息
        const statusElements = this.container.querySelectorAll('.node-status');
        if (statusElements.length >= 3) {
            statusElements[0].textContent = onlineNodes;
            statusElements[1].textContent = relayNodes;
            statusElements[2].textContent = `${health}%`;
        }
    }
}

// 初始化侧边栏
document.addEventListener('DOMContentLoaded', () => {
    if (document.getElementById('sidebar')) {
        window.sidebar = new Sidebar('sidebar');
    }
});