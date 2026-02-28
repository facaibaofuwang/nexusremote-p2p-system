#!/bin/bash
# GitHub Helper - OpenClaw 项目发布辅助脚本
# 使用配置文件中的 GitHub 认证信息

CONFIG_FILE="/home/admin/.openclaw/workspace/github-config.json"

# 读取配置
USERNAME=$(cat "$CONFIG_FILE" | grep -o '"username": "[^"]*"' | cut -d'"' -f4)
TOKEN=$(cat "$CONFIG_FILE" | grep -o '"token": "[^"]*"' | cut -d'"' -f4)
GIT_USER=$(cat "$CONFIG_FILE" | grep -o '"user": "[^"]*"' | cut -d'"' -f4)
GIT_EMAIL=$(cat "$CONFIG_FILE" | grep -o '"email": "[^"]*"' | cut -d'"' -f4)

# 显示当前配置
show_config() {
    echo "=== GitHub 配置 ==="
    echo "用户名: $USERNAME"
    echo "Git 用户: $GIT_USER"
    echo "Git 邮箱: $GIT_EMAIL"
    echo "Token: ${TOKEN:0:10}...${TOKEN: -10}"
    echo "==================="
}

# 创建 GitHub 仓库
create_repo() {
    local repo_name=$1
    local repo_desc=${2:-"使用 OpenClaw 自动化开发"}

    if [ -z "$repo_name" ]; then
        echo "错误: 请提供仓库名称"
        echo "用法: $0 create <仓库名> [描述]"
        exit 1
    fi

    echo "正在创建仓库: $repo_name"
    curl -X POST \
        -H "Authorization: token $TOKEN" \
        -H "Accept: application/vnd.github.v3+json" \
        https://api.github.com/user/repos \
        -d "{
            \"name\": \"$repo_name\",
            \"description\": \"$repo_desc\",
            \"private\": false,
            \"auto_init\": false
        }"

    echo ""
    echo "仓库创建成功: https://github.com/$USERNAME/$repo_name"
}

# 初始化 Git 仓库
init_git() {
    local repo_name=$1
    local project_dir=${2:-.}

    if [ -z "$repo_name" ]; then
        echo "错误: 请提供仓库名称"
        echo "用法: $0 init <仓库名> [项目目录]"
        exit 1
    fi

    cd "$project_dir" || exit 1

    # 初始化 Git 仓库
    if [ ! -d ".git" ]; then
        git init
        git config user.name "$GIT_USER"
        git config user.email "$GIT_EMAIL"
        echo "Git 仓库已初始化"
    fi

    # 添加远程仓库
    git remote add origin "https://$USERNAME@github.com/$USERNAME/$repo_name.git" 2>/dev/null || \
        git remote set-url origin "https://$USERNAME@github.com/$USERNAME/$repo_name.git"

    echo "远程仓库已配置: https://github.com/$USERNAME/$repo_name.git"
}

# 推送到 GitHub
push_to_github() {
    local repo_name=$1
    local project_dir=${2:-.}
    local branch=${3:-main}

    if [ -z "$repo_name" ]; then
        echo "错误: 请提供仓库名称"
        echo "用法: $0 push <仓库名> [项目目录] [分支名]"
        exit 1
    fi

    cd "$project_dir" || exit 1

    # 配置 Git 凭证
    git config credential.helper store
    echo "https://$USERNAME:$TOKEN@github.com" > ~/.git-credentials

    # 推送代码
    echo "正在推送到 GitHub..."
    git push -u origin "$branch"

    if [ $? -eq 0 ]; then
        echo ""
        echo "✅ 推送成功!"
        echo "仓库地址: https://github.com/$USERNAME/$repo_name"
    else
        echo ""
        echo "❌ 推送失败"
        exit 1
    fi
}

# 完整发布流程
publish() {
    local repo_name=$1
    local repo_desc=${2:-"使用 OpenClaw 自动化开发"}
    local project_dir=${3:-.}

    if [ -z "$repo_name" ]; then
        echo "错误: 请提供仓库名称"
        echo "用法: $0 publish <仓库名> [描述] [项目目录]"
        exit 1
    fi

    echo "=== OpenClaw 项目发布 ==="
    echo "仓库名称: $repo_name"
    echo "仓库描述: $repo_desc"
    echo "项目目录: $project_dir"
    echo "========================="
    echo ""

    # 1. 创建仓库
    echo "步骤 1/3: 创建 GitHub 仓库..."
    create_repo "$repo_name" "$repo_desc"
    echo ""

    # 2. 初始化 Git
    echo "步骤 2/3: 初始化 Git 仓库..."
    init_git "$repo_name" "$project_dir"
    echo ""

    # 3. 推送代码
    echo "步骤 3/3: 推送代码到 GitHub..."
    push_to_github "$repo_name" "$project_dir"
    echo ""

    echo "=== 发布完成 ==="
    echo "仓库地址: https://github.com/$USERNAME/$repo_name"
    echo "================="
}

# 显示帮助
show_help() {
    echo "GitHub Helper - OpenClaw 项目发布辅助脚本"
    echo ""
    echo "用法:"
    echo "  $0 config                      - 显示当前配置"
    echo "  $0 create <仓库名> [描述]       - 创建 GitHub 仓库"
    echo "  $0 init <仓库名> [目录]        - 初始化 Git 仓库"
    echo "  $0 push <仓库名> [目录] [分支]  - 推送到 GitHub"
    echo "  $0 publish <仓库名> [描述] [目录] - 完整发布流程"
    echo ""
    echo "示例:"
    echo "  $0 config"
    echo "  $0 create my-project \"我的新项目\""
    echo "  $0 publish my-project \"使用 OpenClaw 自动化开发\" /path/to/project"
}

# 主函数
case "$1" in
    config)
        show_config
        ;;
    create)
        create_repo "$2" "$3"
        ;;
    init)
        init_git "$2" "$3"
        ;;
    push)
        push_to_github "$2" "$3" "$4"
        ;;
    publish)
        publish "$2" "$3" "$4"
        ;;
    *)
        show_help
        ;;
esac
