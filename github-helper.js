#!/usr/bin/env node
/**
 * GitHub Helper - OpenClaw 项目发布辅助脚本 (Node.js 版本)
 * 使用配置文件中的 GitHub 认证信息
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const CONFIG_FILE = '/home/admin/.openclaw/workspace/github-config.json';

// 读取配置
function loadConfig() {
    try {
        const config = JSON.parse(fs.readFileSync(CONFIG_FILE, 'utf8'));
        return config.github;
    } catch (error) {
        console.error('错误: 无法读取配置文件');
        console.error(error.message);
        process.exit(1);
    }
}

// 显示当前配置
function showConfig() {
    const config = loadConfig();
    console.log('=== GitHub 配置 ===');
    console.log(`用户名: ${config.username}`);
    console.log(`Git 用户: ${config.git.user}`);
    console.log(`Git 邮箱: ${config.git.email}`);
    console.log(`Token: ${config.token.substring(0, 10)}...${config.token.substring(config.token.length - 10)}`);
    console.log('===================');
}

// 创建 GitHub 仓库
async function createRepo(repoName, repoDesc) {
    const config = loadConfig();

    if (!repoName) {
        console.error('错误: 请提供仓库名称');
        console.log('用法: node github-helper.js create <仓库名> [描述]');
        process.exit(1);
    }

    const description = repoDesc || '使用 OpenClaw 自动化开发';

    console.log(`正在创建仓库: ${repoName}`);

    try {
        const curlCommand = `curl -X POST \
            -H "Authorization: token ${config.token}" \
            -H "Accept: application/vnd.github.v3+json" \
            ${config.apiUrl}/user/repos \
            -d '{
                "name": "${repoName}",
                "description": "${description}",
                "private": false,
                "auto_init": false
            }'`;

        const result = execSync(curlCommand, { encoding: 'utf8' });
        const repo = JSON.parse(result);

        console.log('');
        console.log('✅ 仓库创建成功!');
        console.log(`仓库地址: https://github.com/${config.username}/${repoName}`);
        console.log(`仓库 ID: ${repo.id}`);
        return repo;
    } catch (error) {
        console.error('❌ 创建仓库失败');
        console.error(error.message);
        process.exit(1);
    }
}

// 初始化 Git 仓库
function initGit(repoName, projectDir) {
    const config = loadConfig();

    if (!repoName) {
        console.error('错误: 请提供仓库名称');
        console.log('用法: node github-helper.js init <仓库名> [项目目录]');
        process.exit(1);
    }

    const dir = projectDir || process.cwd();

    try {
        process.chdir(dir);

        // 初始化 Git 仓库
        if (!fs.existsSync('.git')) {
            execSync('git init', { stdio: 'inherit' });
            execSync(`git config user.name "${config.git.user}"`, { stdio: 'inherit' });
            execSync(`git config user.email "${config.git.email}"`, { stdio: 'inherit' });
            console.log('✅ Git 仓库已初始化');
        }

        // 添加远程仓库
        const remoteUrl = `https://${config.username}@github.com/${config.username}/${repoName}.git`;
        try {
            execSync(`git remote add origin ${remoteUrl}`, { stdio: 'inherit' });
        } catch (error) {
            // 远程仓库可能已存在，更新它
            execSync(`git remote set-url origin ${remoteUrl}`, { stdio: 'inherit' });
        }

        console.log(`✅ 远程仓库已配置: https://github.com/${config.username}/${repoName}.git`);
    } catch (error) {
        console.error('❌ 初始化 Git 失败');
        console.error(error.message);
        process.exit(1);
    }
}

// 推送到 GitHub
function pushToGithub(repoName, projectDir, branch) {
    const config = loadConfig();

    if (!repoName) {
        console.error('错误: 请提供仓库名称');
        console.log('用法: node github-helper.js push <仓库名> [项目目录] [分支名]');
        process.exit(1);
    }

    const dir = projectDir || process.cwd();
    const branchName = branch || 'main';

    try {
        process.chdir(dir);

        // 配置 Git 凭证
        execSync('git config credential.helper store', { stdio: 'inherit' });
        const credentials = `https://${config.username}:${config.token}@github.com`;
        fs.writeFileSync(path.join(process.env.HOME, '.git-credentials'), credentials);

        // 推送代码
        console.log('正在推送到 GitHub...');
        execSync(`git push -u origin ${branchName}`, { stdio: 'inherit' });

        console.log('');
        console.log('✅ 推送成功!');
        console.log(`仓库地址: https://github.com/${config.username}/${repoName}`);
    } catch (error) {
        console.error('❌ 推送失败');
        console.error(error.message);
        process.exit(1);
    }
}

// 完整发布流程
async function publish(repoName, repoDesc, projectDir) {
    const config = loadConfig();

    if (!repoName) {
        console.error('错误: 请提供仓库名称');
        console.log('用法: node github-helper.js publish <仓库名> [描述] [项目目录]');
        process.exit(1);
    }

    const description = repoDesc || '使用 OpenClaw 自动化开发';
    const dir = projectDir || process.cwd();

    console.log('=== OpenClaw 项目发布 ===');
    console.log(`仓库名称: ${repoName}`);
    console.log(`仓库描述: ${description}`);
    console.log(`项目目录: ${dir}`);
    console.log('=========================');
    console.log('');

    // 1. 创建仓库
    console.log('步骤 1/3: 创建 GitHub 仓库...');
    await createRepo(repoName, description);
    console.log('');

    // 2. 初始化 Git
    console.log('步骤 2/3: 初始化 Git 仓库...');
    initGit(repoName, dir);
    console.log('');

    // 3. 推送代码
    console.log('步骤 3/3: 推送代码到 GitHub...');
    pushToGithub(repoName, dir);
    console.log('');

    console.log('=== 发布完成 ===');
    console.log(`仓库地址: https://github.com/${config.username}/${repoName}`);
    console.log('=================');
}

// 显示帮助
function showHelp() {
    console.log('GitHub Helper - OpenClaw 项目发布辅助脚本 (Node.js 版本)');
    console.log('');
    console.log('用法:');
    console.log('  node github-helper.js config                      - 显示当前配置');
    console.log('  node github-helper.js create <仓库名> [描述]       - 创建 GitHub 仓库');
    console.log('  node github-helper.js init <仓库名> [目录]        - 初始化 Git 仓库');
    console.log('  node github-helper.js push <仓库名> [目录] [分支]  - 推送到 GitHub');
    console.log('  node github-helper.js publish <仓库名> [描述] [目录] - 完整发布流程');
    console.log('');
    console.log('示例:');
    console.log('  node github-helper.js config');
    console.log('  node github-helper.js create my-project "我的新项目"');
    console.log('  node github-helper.js publish my-project "使用 OpenClaw 自动化开发" /path/to/project');
}

// 主函数
const command = process.argv[2];
const args = process.argv.slice(3);

switch (command) {
    case 'config':
        showConfig();
        break;
    case 'create':
        createRepo(args[0], args[1]);
        break;
    case 'init':
        initGit(args[0], args[1]);
        break;
    case 'push':
        pushToGithub(args[0], args[1], args[2]);
        break;
    case 'publish':
        publish(args[0], args[1], args[2]);
        break;
    default:
        showHelp();
}
