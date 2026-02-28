// 顶部工具栏组件
class Topbar {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.notifications = [];
        this.init();
    }
    
    init() {
        this.render();
        this.bindEvents();
        this.loadNotifications();
    }
    
    render() {
        this.container.innerHTML = `
            <header class="bg-secondary border-b border-dark-lighter px-6 py-4">
                <div class="flex items-center justify-between">
                    <!-- 左侧：面包屑和搜索 -->
                    <div class="flex items-center space-x-4">
                        <!-- 面包屑导航 -->
                        <div class="flex items-center space-x-2 text-sm">
                            <span class="text-gray-400">NexusRemote</span>
                            <i class="fa fa-chevron-right text-gray-600 text-xs"></i>
                            <span class="text-white">仪表盘</span>
                        </div>
                        
                        <!-- 搜索框 -->
                        <div class="hidden md:block">
                            <div class="relative">
                                <input type="text" 
                                       placeholder="搜索设备、节点或交易..." 
                                       class="bg-dark-light text-sm rounded-lg pl-10 pr-4 py-2 w-64 focus:outline-none focus:ring-2 focus:ring-accent focus:border-transparent"
                                       id="searchInput">
                                <i class="fa fa-search absolute left-3 top-2.5 text-gray-500"></i>
                            </div>
                        </div>
                    </div>
                    
                    <!-- 右侧：通知和用户操作 -->
                    <div class="flex items-center space-x-4">
                        <!-- 通知 -->
                        <div class="relative" id="notificationContainer">
                            <button class="relative p-2 rounded-lg hover:bg-dark-lighter transition-colors" id="notificationButton">
                                <i class="fa fa-bell text-gray-400 hover:text-white"></i>
                                ${this.notifications.length > 0 ? `
                                <span class="absolute -top-1 -right-1 w-4 h-4 bg-danger rounded-full text-xs flex items-center justify-center">
                                    ${this.notifications.length}
                                </span>
                                ` : ''}
                            </button>
                            <!-- 通知下拉菜单 -->
                            <div class="absolute right-0 top-full mt-2 w-80 bg-secondary rounded-lg shadow-lg border border-dark-lighter hidden z-50" 
                                 id="notificationDropdown">
                                <div class="p-4 border-b border-dark-lighter">
                                    <div class="flex justify-between items-center">
                                        <h3 class="font-medium text-white">通知</h3>
                                        <button class="text-xs text-accent hover:text-blue-400" id="markAllRead">
                                            全部标记为已读
                                        </button>
                                    </div>
                                </div>
                                <div class="max-h-80 overflow-y-auto" id="notificationList">
                                    ${this.renderNotifications()}
                                </div>
                                <div class="p-3 border-t border-dark-lighter text-center">
                                    <a href="#" class="text-sm text-accent hover:text-blue-400">查看所有通知</a>
                                </div>
                            </div>
                        </div>
                        
                        <!-- 主题切换 -->
                        <button class="p-2 rounded-lg hover:bg-dark-lighter transition-colors" id="themeToggle">
                            <i class="fa fa-moon text-gray-400 hover:text-white"></i>
                        </button>
                        
                        <!-- 网络状态 -->
                        <div class="hidden md:flex items-center space-x-2 px-3 py-2 rounded-lg bg-dark-light">
                            <div class="w-2 h-2 rounded-full bg-success"></div>
                            <span class="text-sm">网络正常</span>
                        </div>
                        
                        <!-- 快速操作 -->
                        <div class="flex items-center space-x-2">
                            <button class="px-4 py-2 bg-accent text-white rounded-lg hover:bg-blue-600 transition-colors flex items-center">
                                <i class="fa fa-plus mr-2"></i>
                                <span>快速连接</span>
                            </button>
                            <button class="p-2 rounded-lg hover:bg-dark-lighter transition-colors">
                                <i class="fa fa-question-circle text-gray-400 hover:text-white"></i>
                            </button>
                        </div>
                    </div>
                </div>
                
                <!-- 移动端搜索 -->
                <div class="md:hidden mt-4">
                    <div class="relative">
                        <input type="text" 
                               placeholder="搜索设备、节点或交易..." 
                               class="bg-dark-light text-sm rounded-lg pl-10 pr-4 py-2 w-full focus:outline-none focus:ring-2 focus:ring-accent focus:border-transparent">
                        <i class="fa fa-search absolute left-3 top-2.5 text-gray-500"></i>
                    </div>
                </div>
            </header>
        `;
    }
    
    renderNotifications() {
        if (this.notifications.length === 0) {
            return `
                <div class="p-4 text-center text-gray-500">
                    <i class="fa fa-bell-slash text-2xl mb-2"></i>
                    <p>没有新通知</p>
                </div>
            `;
        }
        
        return this.notifications.map(notification => `
            <div class="p-4 border-b border-dark-lighter hover:bg-dark-lighter transition-colors ${notification.unread ? 'bg-dark-light' : ''}">
                <div class="flex justify-between items-start mb-1">
                    <span class="text-sm font-medium text-white">${notification.title}</span>
                    <span class="text-xs text-gray-500">${notification.time}</span>
                </div>
                <p class="text-xs text-gray-400">${notification.message}</p>
                ${notification.action ? `
                <button class="mt-2 text-xs text-accent hover:text-blue-400" data-action="${notification.action}">
                    ${notification.actionText || '查看详情'}
                </button>
                ` : ''}
            </div>
        `).join('');
    }
    
    bindEvents() {
        // 通知下拉菜单
        const notificationButton = document.getElementById('notificationButton');
        const notificationDropdown = document.getElementById('notificationDropdown');
        
        if (notificationButton && notificationDropdown) {
            notificationButton.addEventListener('click', (e) => {
                e.stopPropagation();
                notificationDropdown.classList.toggle('hidden');
            });
            
            // 点击外部关闭下拉菜单
            document.addEventListener('click', (e) => {
                if (!notificationContainer.contains(e.target)) {
                    notificationDropdown.classList.add('hidden');
                }
            });
        }
        
        // 标记所有为已读
        const markAllRead = document.getElementById('markAllRead');
        if (markAllRead) {
            markAllRead.addEventListener('click', () => {
                this.markAllNotificationsAsRead();
            });
        }
        
        // 主题切换
        const themeToggle = document.getElementById('themeToggle');
        if (themeToggle) {
            themeToggle.addEventListener('click', () => {
                this.toggleTheme();
            });
        }
        
        // 搜索功能
        const searchInput = document.getElementById('searchInput');
        if (searchInput) {
            searchInput.addEventListener('input', (e) => {
                this.handleSearch(e.target.value);
            });
            
            searchInput.addEventListener('keypress', (e) => {
                if (e.key === 'Enter') {
                    this.performSearch(e.target.value);
                }
            });
        }
    }
    
    loadNotifications() {
        // 模拟加载通知
        this.notifications = [
            {
                id: 1,
                title: '新设备连接',
                message: '设备 "办公室电脑" 已成功连接到网络',
                time: '刚刚',
                unread: true,
                action: 'viewDevice',
                actionText: '查看设备'
            },
            {
                id: 2,
                title: '通证交易完成',
                message: '成功收到 12.5 NEXUS 通证',
                time: '10分钟前',
                unread: true,
                action: 'viewTransaction',
                actionText: '查看交易'
            },
            {
                id: 3,
                title: '网络优化建议',
                message: '检测到网络延迟较高，建议优化路由配置',
                time: '1小时前',
                unread: false,
                action: 'viewNetwork',
                actionText: '查看网络'
            }
        ];
        
        // 如果有通知容器，更新通知显示
        const notificationList = document.getElementById('notificationList');
        if (notificationList) {
            notificationList.innerHTML = this.renderNotifications();
        }
    }
    
    addNotification(notification) {
        notification.id = Date.now();
        notification.unread = true;
        notification.time = '刚刚';
        
        this.notifications.unshift(notification);
        this.updateNotificationBadge();
        
        // 更新通知列表
        const notificationList = document.getElementById('notificationList');
        if (notificationList) {
            notificationList.innerHTML = this.renderNotifications();
        }
    }
    
    markAllNotificationsAsRead() {
        this.notifications.forEach(notification => {
            notification.unread = false;
        });
        this.updateNotificationBadge();
        
        // 更新通知列表
        const notificationList = document.getElementById('notificationList');
        if (notificationList) {
            notificationList.innerHTML = this.renderNotifications();
        }
    }
    
    updateNotificationBadge() {
        const unreadCount = this.notifications.filter(n => n.unread).length;
        const notificationButton = document.getElementById('notificationButton');
        
        if (notificationButton) {
            let badge = notificationButton.querySelector('.absolute');
            if (unreadCount > 0) {
                if (!badge) {
                    badge = document.createElement('span');
                    badge.className = 'absolute -top-1 -right-1 w-4 h-4 bg-danger rounded-full text-xs flex items-center justify-center';
                    notificationButton.appendChild(badge);
                }
                badge.textContent = unreadCount;
            } else if (badge) {
                badge.remove();
            }
        }
    }
    
    toggleTheme() {
        const html = document.documentElement;
        const themeToggle = document.getElementById('themeToggle');
        const icon = themeToggle.querySelector('i');
        
        if (html.classList.contains('dark')) {
            html.classList.remove('dark');
            icon.classList.remove('fa-sun');
            icon.classList.add('fa-moon');
            localStorage.setItem('theme', 'light');
        } else {
            html.classList.add('dark');
            icon.classList.remove('fa-moon');
            icon.classList.add('fa-sun');
            localStorage.setItem('theme', 'dark');
        }
    }
    
    handleSearch(query) {
        // 实时搜索建议
        if (query.length > 2) {
            // 这里可以实现搜索建议功能
            console.log('搜索建议:', query);
        }
    }
    
    performSearch(query) {
        if (query.trim()) {
            // 执行搜索
            console.log('执行搜索:', query);
            // 可以触发搜索事件或跳转到搜索结果页面
            const event = new CustomEvent('search', { detail: { query } });
            document.dispatchEvent(event);
        }
    }
}

// 初始化顶部工具栏
document.addEventListener('DOMContentLoaded', () => {
    if (document.getElementById('topbar')) {
        window.topbar = new Topbar('topbar');
    }
});