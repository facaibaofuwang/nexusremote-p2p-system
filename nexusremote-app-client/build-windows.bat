@echo off
REM NexusRemote Windows 打包脚本
REM 用于生成Windows安装包

echo ====================================
echo NexusRemote Windows 打包脚本
echo ====================================
echo.

REM 检查Node.js
where node >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [错误] 未找到Node.js，请先安装Node.js
    echo 下载: https://nodejs.org/
    pause
    exit /b 1
)

REM 检查npm
where npm >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [错误] 未找到npm，请先安装npm
    pause
    exit /b 1
)

echo [信息] Node.js版本:
node --version
echo [信息] npm版本:
npm --version
echo.

REM 进入项目目录
cd nexusremote-app-client
if %ERRORLEVEL% NEQ 0 (
    echo [错误] 无法进入nexusremote-app-client目录
    pause
    exit /b 1
)

echo [步骤1] 安装依赖...
echo 正在安装npm依赖...
call npm install
if %ERRORLEVEL% NEQ 0 (
    echo [错误] 依赖安装失败
    pause
    exit /b 1
)
echo [完成] 依赖安装完成
echo.

REM 编译Rust后端为Windows
echo [步骤2] 编译Rust后端...
cd ..\nexusremote
echo 正在编译Rust项目（Windows目标）...

REM 检查Cargo
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [警告] 未找到Cargo，跳过Rust编译
    echo [信息] 将使用预构建的二进制文件
) else (
    echo 编译nexusremote.exe（x86_64-pc-windows-msvc）...
    call cargo build --release --target x86_64-pc-windows-msvc
    if %ERRORLEVEL% NEQ 0 (
        echo [警告] Rust编译失败
        echo [信息] 检查是否安装了Visual Studio Build Tools
    ) else (
        echo [完成] Rust编译成功
    )
)

echo.

REM 复制Rust二进制文件
echo [步骤3] 复制编译后的文件...
cd ..\nexusremote-app-client

if exist "..\nexusremote\target\release\nexusremote.exe" (
    echo 复制nexusremote.exe...
    xcopy "..\nexusremote\target\release\nexusremote.exe" "nexusremote.exe*" /Y
    echo [完成] Rust二进制文件复制完成
) else (
    echo [警告] 未找到nexusremote.exe，跳过复制
)

echo.

REM 创建图标资源
echo [步骤4] 准备图标资源...
if not exist "assets\icons" mkdir "assets\icons"

REM 如果没有图标文件，创建简单的占位符
if not exist "assets\icons\icon.ico" (
    echo [信息] 创建占位图标（需要替换为真实图标）
    echo. > "assets\icons\icon.ico"
)

if not exist "assets\icons\icon.png" (
    echo [信息] 创建占位PNG图标（需要替换为真实图标）
    echo. > "assets\icons\icon.png"
)

if not exist "assets\icons\bitmap.bmp" (
    echo [信息] 创建占位位图（需要替换为真实位图）
    echo. > "assets\icons\bitmap.bmp"
)

echo [完成] 资源文件准备完成
echo.

REM 创建许可证文件
echo [步骤5] 准备许可证...
if exist "..\LICENSE" (
    xcopy "..\LICENSE" "LICENSE" /Y
    echo [完成] 许可证文件复制完成
) else (
    echo [警告] 未找到LICENSE文件
)
echo.

REM 打包Windows安装程序
echo [步骤6] 构建Windows安装包...
echo 正在构建NSIS安装程序...
call npm run build:win

if %ERRORLEVEL% NEQ 0 (
    echo [错误] Windows打包失败
    echo.
    echo 可能的原因：
    echo 1. 未安装electron-builder
    echo 2. 缺少必要的依赖
    echo 3. 图标资源缺失
    echo.
    pause
    exit /b 1
)

echo.
echo ====================================
echo [成功] Windows安装包构建完成！
echo ====================================
echo.
echo 生成的文件位置：
dir /b dist\*.exe 2>nul
if %ERRORLEVEL% EQU 0 (
    dir /b dist\*.exe
) else (
    echo [信息] 检查dist目录中的安装包
    dir dist
)

echo.
echo 安装包特性：
echo - NSIS安装程序（专业安装向导）
echo - 可选的便携版本
echo - 桌面快捷方式
echo - 开始菜单快捷方式
echo - 自动启动项
echo.

echo ====================================
echo 使用说明：
echo ====================================
echo.
echo 1. 将生成的.exe文件传输到Windows电脑
echo 2. 双击运行安装程序
echo 3. 按照安装向导完成安装
echo 4. 安装完成后可从开始菜单启动
echo.

pause
