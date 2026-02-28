// 状态卡片组件
class StatusCards {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.data = {
            tokenBalance: 1248.5,
            tokenChange: 12.5,
            networkStatus: '良好',
            networkDetails: {
                latency: 12,
                bandwidth: 50,
                packetLoss: 0.1,
                nodes: 127
            },
            devices: {
                online: 5,
                total: 8,
                cpuUsage: 45,
                memoryUsage: 62
            }
        };
        this.init();
    }
    
    init() {
        this.render();
        this.startLiveUpdates();
    }
    
    render() {
        this.container.innerHTML = `
            <!-- 通证余额 -->
            <div class="bg-secondary rounded-lg p-5 border-l-4 border-accent card-hover">
                <div class="flex justify-between items-start">
                    <div>
                        <p class="text-sm font-medium text-gray-400">通证余额</p>
                        <h3 class="text-3xl font-bold text-white mt-1">${this.formatNumber(this.data.tokenBalance)} NEXUS</h3>
                        <p class="text-xs ${this.data.tokenChange >= 0 ? 'text-success' : 'text-danger'} flex items-center mt-1">
                            <i class="fa fa-arrow-${this.data.tokenChange >= 0 ? 'up' : 'down'} mr-1"></i>
                            <span>${this.data.tokenChange >= 0 ? '+' : ''}${this.data.tokenChange}% 本周</span>
                        </p>
                    </div>
                    <div class="w-12 h-12 rounded-full bg-accent bg-opacity-20 flex items-center justify-center">
                        <i class="fa fa-money text-accent text-xl"></i>
                    </div>
                </div>
                <div class="mt-4">
                    <div class="progress-bar">
                        <div class="progress-fill bg-accent" style="width: ${(this.data.tokenBalance / 2000) * 100}%"></div>
                    </div>
                    <div class="flex justify-between text-xs text-gray-500 mt-1">
                        <span>可用余额</span>
                        <span>最大容量: 2,000 NEXUS</span>
                    </div>
                </div>
            </div>

            <!-- 网络状态 -->
            <div class="bg-secondary rounded-lg p-5 border-l-4 border-success card-hover">
                <div class="flex justify-between items-start">
                    <div>
                        <p class="text-sm font-medium text-gray-400">网络状态</p>
                        <h3 class="text-3xl font-bold text-white mt-1">${this.data.networkStatus}</h3>
                        <p class="text-xs text-success flex items-center mt-1">
                            <i class="fa fa-signal mr-1"></i>
                            <span>低延迟，高带宽</span>
                        </p>
                    </div>
                    <div class="w-12 h-12 rounded-full bg-success bg-opacity-20 flex items-center justify-center">
                        <i class="fa fa-wifi text-success text-xl"></i>
                    </div>
                </div>
                <div class="mt-4 grid grid-cols-2 gap-4">
                    <div>
                        <p class="text-xs text-gray-500">延迟</p>
                        <p class="text-sm font-medium text-white">${this.data.networkDetails.latency}ms</p>
                    </div>
                    <div>
                        <p class="text-xs text-gray-500">带宽</p>
                        <p class="text-sm font-medium text-white">${this.data.networkDetails.bandwidth}Mbps</p>
                    </div>
                    <div>
                        <p class="text-xs text-gray-500">丢包率</p>
                        <p class="text-sm font-medium text-white">${this.data.networkDetails.packetLoss}%</p>
                    </div>
                    <div>
                        <p class="text-xs text-gray-500">节点数</p>
                        <p class="text-sm font-medium text-white">${this.data.networkDetails.nodes}</p>
                    </div>
                </div>
            </div>

            <!-- 设备状态 -->
            <div class="bg-secondary rounded-lg p-5 border-l-4 border-warning card-hover">
                <div class="flex justify-between items-start">
                    <div>
                        <p class="text-sm font-medium text-gray-400">设备状态</p>
                        <h3 class="text-3xl font-bold text-white mt-1">${this.data.devices.online} 台在线</h3>
                        <p class="text-xs ${this.data.devices.online === this.data.devices.total ? 'text-success' : 'text-warning'} flex items-center mt-1">
                            <i class="fa fa-desktop mr-1"></i>
                            <span>${this.data.devices.total - this.data.devices.online} 台需要注意</span>
                        </p>
                    </div>
                    <div class="w-12 h-12 rounded-full bg-warning bg-opacity-20 flex items-center justify-center">
                        <i class="fa fa-laptop text-warning text-xl"></i>
                    </div>
                </div>
                <div class="mt-4">
                    <div class="flex items-center justify-between mb-2">
                        <span class="text-xs text-gray-500">CPU 使用率</span>
                        <span class="text-xs text-gray-500">${this.data.devices.cpuUsage}%</span>
                    </div>
                    <div class="progress-bar">
                        <div class="progress-fill bg-warning" style="width: ${this.data.devices.cpuUsage}%"></div>
                    </div>
                    <div class="flex items-center justify-between mb-2 mt-3">
                        <span class="text-xs text-gray-500">内存使用率</span>
                        <span class="text-xs text-gray-500">${this.data.devices.memoryUsage}%</span>
                    </div>
                    <div class="progress-bar">
                        <div class="progress-fill bg-warning" style="width: ${this.data.devices.memoryUsage}%"></div>
                    </div>
                </div>
            </div>
        `;
    }
    
    formatNumber(num) {
        return num.toLocaleString('en-US', {
            minimumFractionDigits: 1,
            maximumFractionDigits: 1
        });
    }
    
    updateData(newData) {
        // 合并新数据
        this.data = { ...this.data, ...newData };
        this.render();
    }
    
    updateTokenBalance(balance, change) {
        this.data.tokenBalance = balance;
        this.data.tokenChange = change;
        this.render();
    }
    
    updateNetworkStatus(status, details) {
        this.data.networkStatus = status;
        if (details) {
            this.data.networkDetails = { ...this.data.networkDetails, ...details };
        }
        this.render();
    }
    
    updateDeviceStatus(online, total, cpuUsage, memoryUsage) {
        this.data.devices = {
            online: online || this.data.devices.online,
            total: total || this.data.devices.total,
            cpuUsage: cpuUsage || this.data.devices.cpuUsage,
            memoryUsage: memoryUsage || this.data.devices.memoryUsage
        };
        this.render();
    }
    
    startLiveUpdates() {
        // 模拟实时数据更新
        setInterval(() => {
            this.simulateLiveData();
        }, 10000); // 每10秒更新一次
    }
    
    simulateLiveData() {
        // 模拟数据变化
        const randomChange = (Math.random() - 0.5) * 0.5; // -0.25 到 +0.25
        this.data.tokenBalance += randomChange;
        
        // 网络状态随机波动
        this.data.networkDetails.latency = Math.max(5, Math.min(50, 
            this.data.networkDetails.latency + (Math.random() - 0.5) * 5
        ));
        
        this.data.networkDetails.bandwidth = Math.max(10, Math.min(100,
            this.data.networkDetails.bandwidth + (Math.random() - 0.5) * 10
        ));
        
        this.data.networkDetails.packetLoss = Math.max(0, Math.min(2,
            this.data.networkDetails.packetLoss + (Math.random() - 0.5) * 0.2
        ));
        
        // 设备状态随机波动
        this.data.devices.cpuUsage = Math.max(10, Math.min(90,
            this.data.devices.cpuUsage + (Math.random() - 0.5) * 10
        ));
        
        this.data.devices.memoryUsage = Math.max(20, Math.min(90,
            this.data.devices.memoryUsage + (Math.random() - 0.5) * 5
        ));
        
        // 渲染更新
        this.render();
        
        // 触发数据更新事件
        const event = new CustomEvent('statusUpdate', { detail: this.data });
        document.dispatchEvent(event);
    }
    
    // 获取当前数据
    getData() {
        return this.data;
    }
}

// 初始化状态卡片
document.addEventListener('DOMContentLoaded', () => {
    if (document.getElementById('statusCards')) {
        window.statusCards = new StatusCards('statusCards');
    }
});