const { app, BrowserWindow, Menu, Tray, ipcMain, dialog, shell } = require('electron');
const path = require('path');
const Store = require('electron-store');
const { autoUpdater } = require('electron-updater');

// 配置存储
const store = new Store({
  defaults: {
    windowBounds: { width: 1200, height: 800 },
    serverConfig: {
      pythonBackend: 'http://localhost:5000',
      rustWebSocket: 'ws://localhost:8081',
      frontend: 'http://localhost:3000'
    },
    connectionStatus: {
      pythonBackend: false,
      rustWebSocket: false,
      frontend: false
    },
    theme: 'dark',
    autoConnect: true,
    notifications: true
  }
});

let mainWindow;
let tray = null;

// 创建主窗口
function createWindow() {
  const { width, height } = store.get('windowBounds');
  
  mainWindow = new BrowserWindow({
    width,
    height,
    minWidth: 800,
    minHeight: 600,
    icon: path.join(__dirname, 'assets/icons/icon.png'),
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false,
      enableRemoteModule: true,
      webSecurity: false
    },
    frame: true,
    titleBarStyle: 'hiddenInset',
    backgroundColor: '#0f172a',
    show: false
  });

  // 加载应用界面
  mainWindow.loadFile('index.html');

  // 窗口准备好后显示
  mainWindow.once('ready-to-show', () => {
    mainWindow.show();
    
    // 检查更新
    if (process.env.NODE_ENV !== 'development') {
      autoUpdater.checkForUpdatesAndNotify();
    }
  });

  // 保存窗口大小
  mainWindow.on('resize', () => {
    const { width, height } = mainWindow.getBounds();
    store.set('windowBounds', { width, height });
  });

  // 窗口关闭事件
  mainWindow.on('close', (event) => {
    if (app.quit) {
      return;
    }
    
    event.preventDefault();
    mainWindow.hide();
    
    // 显示通知
    if (store.get('notifications')) {
      const notification = {
        title: 'NexusRemote 仍在运行',
        body: '应用程序已最小化到系统托盘。右键点击托盘图标可退出。',
        icon: path.join(__dirname, 'assets/icons/icon.png')
      };
      new Notification(notification.title, notification).show();
    }
  });

  // 开发者工具
  if (process.env.NODE_ENV === 'development') {
    mainWindow.webContents.openDevTools();
  }
}

// 创建系统托盘
function createTray() {
  tray = new Tray(path.join(__dirname, 'assets/icons/tray.png'));
  
  const contextMenu = Menu.buildFromTemplate([
    {
      label: '打开 NexusRemote',
      click: () => {
        if (mainWindow) {
          mainWindow.show();
        }
      }
    },
    {
      label: '连接状态',
      submenu: [
        {
          label: 'Python后端: 检查中...',
          id: 'python-status',
          enabled: false
        },
        {
          label: 'Rust WebSocket: 检查中...',
          id: 'rust-status',
          enabled: false
        },
        {
          label: '前端服务: 检查中...',
          id: 'frontend-status',
          enabled: false
        }
      ]
    },
    { type: 'separator' },
    {
      label: '设置',
      click: () => {
        if (mainWindow) {
          mainWindow.webContents.send('open-settings');
          mainWindow.show();
        }
      }
    },
    { type: 'separator' },
    {
      label: '检查更新',
      click: () => {
        autoUpdater.checkForUpdates();
      }
    },
    {
      label: '关于',
      click: () => {
        dialog.showMessageBox(mainWindow, {
          type: 'info',
          title: '关于 NexusRemote',
          message: 'NexusRemote 桌面客户端',
          detail: `版本: ${app.getVersion()}\n作者: NexusRemote Team\n许可证: MIT\nGitHub: https://github.com/facaibaofuwang/nexusremote-p2p-system`,
          buttons: ['打开GitHub', '确定'],
          defaultId: 1
        }).then((result) => {
          if (result.response === 0) {
            shell.openExternal('https://github.com/facaibaofuwang/nexusremote-p2p-system');
          }
        });
      }
    },
    { type: 'separator' },
    {
      label: '退出',
      click: () => {
        app.quit = true;
        app.quit();
      }
    }
  ]);
  
  tray.setToolTip('NexusRemote - 去中心化P2P远程控制系统');
  tray.setContextMenu(contextMenu);
  
  // 托盘图标点击事件
  tray.on('click', () => {
    if (mainWindow) {
      if (mainWindow.isVisible()) {
        mainWindow.hide();
      } else {
        mainWindow.show();
      }
    }
  });
}

// 更新托盘状态
function updateTrayStatus(service, status) {
  if (!tray) return;
  
  const contextMenu = tray.getContextMenu();
  const menuItem = contextMenu.getMenuItemById(`${service}-status`);
  
  if (menuItem) {
    const statusText = status ? '✅ 已连接' : '❌ 未连接';
    menuItem.label = `${service}: ${statusText}`;
    
    // 更新存储状态
    const connectionStatus = store.get('connectionStatus');
    connectionStatus[service] = status;
    store.set('connectionStatus', connectionStatus);
    
    // 更新托盘工具提示
    const allConnected = Object.values(connectionStatus).every(s => s);
    tray.setToolTip(`NexusRemote - ${allConnected ? '全部连接正常' : '部分服务未连接'}`);
  }
}

// IPC通信处理
ipcMain.handle('get-config', () => {
  return store.get('serverConfig');
});

ipcMain.handle('save-config', (event, config) => {
  store.set('serverConfig', config);
  return { success: true };
});

ipcMain.handle('test-connection', async (event, service, url) => {
  try {
    let connected = false;
    
    switch(service) {
      case 'pythonBackend':
        const { default: axios } = await import('axios');
        const response = await axios.get(`${url}/api/health`, { timeout: 5000 });
        connected = response.status === 200 && response.data.status === 'healthy';
        break;
        
      case 'rustWebSocket':
        // WebSocket连接测试
        const WebSocket = require('ws');
        const ws = new WebSocket(url);
        
        connected = await new Promise((resolve) => {
          const timeout = setTimeout(() => {
            ws.close();
            resolve(false);
          }, 5000);
          
          ws.on('open', () => {
            clearTimeout(timeout);
            ws.close();
            resolve(true);
          });
          
          ws.on('error', () => {
            clearTimeout(timeout);
            resolve(false);
          });
        });
        break;
        
      case 'frontend':
        const { default: axios2 } = await import('axios');
        const response2 = await axios2.get(`${url}/api/devices`, { timeout: 5000 });
        connected = response2.status === 200;
        break;
    }
    
    // 更新托盘状态
    updateTrayStatus(service, connected);
    
    return { connected, service };
  } catch (error) {
    updateTrayStatus(service, false);
    return { connected: false, service, error: error.message };
  }
});

ipcMain.handle('get-connection-status', () => {
  return store.get('connectionStatus');
});

ipcMain.handle('open-external', (event, url) => {
  shell.openExternal(url);
});

// 自动更新事件
autoUpdater.on('update-available', () => {
  if (mainWindow) {
    mainWindow.webContents.send('update-available');
  }
});

autoUpdater.on('update-downloaded', () => {
  if (mainWindow) {
    mainWindow.webContents.send('update-downloaded');
  }
});

autoUpdater.on('error', (err) => {
  console.error('更新错误:', err);
});

// 应用生命周期
app.whenReady().then(() => {
  createWindow();
  createTray();
  
  // 自动连接检查
  if (store.get('autoConnect')) {
    setTimeout(() => {
      const config = store.get('serverConfig');
      Object.keys(config).forEach(service => {
        mainWindow.webContents.send('test-connection', service, config[service]);
      });
    }, 2000);
  }
  
  // 创建应用菜单
  const menu = Menu.buildFromTemplate([
    {
      label: '文件',
      submenu: [
        {
          label: '设置',
          accelerator: 'CmdOrCtrl+,',
          click: () => {
            mainWindow.webContents.send('open-settings');
          }
        },
        { type: 'separator' },
        {
          label: '重新加载',
          accelerator: 'CmdOrCtrl+R',
          click: () => {
            mainWindow.reload();
          }
        },
        {
          label: '开发者工具',
          accelerator: process.platform === 'darwin' ? 'Alt+Command+I' : 'Ctrl+Shift+I',
          click: () => {
            mainWindow.webContents.openDevTools();
          }
        },
        { type: 'separator' },
        {
          label: '退出',
          accelerator: 'CmdOrCtrl+Q',
          click: () => {
            app.quit = true;
            app.quit();
          }
        }
      ]
    },
    {
      label: '视图',
      submenu: [
        {
          label: '放大',
          accelerator: 'CmdOrCtrl+=',
          click: () => {
            mainWindow.webContents.setZoomLevel(mainWindow.webContents.getZoomLevel() + 1);
          }
        },
        {
          label: '缩小',
          accelerator: 'CmdOrCtrl+-',
          click: () => {
            mainWindow.webContents.setZoomLevel(mainWindow.webContents.getZoomLevel() - 1);
          }
        },
        {
          label: '重置缩放',
          accelerator: 'CmdOrCtrl+0',
          click: () => {
            mainWindow.webContents.setZoomLevel(0);
          }
        }
      ]
    },
    {
      label: '连接',
      submenu: [
        {
          label: '测试所有连接',
          accelerator: 'CmdOrCtrl+T',
          click: () => {
            const config = store.get('serverConfig');
            Object.keys(config).forEach(service => {
              mainWindow.webContents.send('test-connection', service, config[service]);
            });
          }
        },
        {
          label: '打开Web界面',
          accelerator: 'CmdOrCtrl+W',
          click: () => {
            const config = store.get('serverConfig');
            shell.openExternal(config.frontend);
          }
        }
      ]
    },
    {
      label: '帮助',
      submenu: [
        {
          label: '文档',
          click: () => {
            shell.openExternal('https://github.com/facaibaofuwang/nexusremote-p2p-system');
          }
        },
        {
          label: '报告问题',
          click: () => {
            shell.openExternal('https://github.com/facaibaofuwang/nexusremote-p2p-system/issues');
          }
        },
        { type: 'separator' },
        {
          label: '关于 NexusRemote',
          click: () => {
            dialog.showMessageBox(mainWindow, {
              type: 'info',
              title: '关于 NexusRemote',
              message: 'NexusRemote 桌面客户端',
              detail: `版本: ${app.getVersion()}\n作者: NexusRemote Team\n许可证: MIT`,
              buttons: ['确定']
            });
          }
        }
      ]
    }
  ]);
  
  Menu.setApplicationMenu(menu);
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});

app.on('before-quit', () => {
  app.quit = true;
});