#!/bin/bash

# ==============================================================================
# Orion 本地一键部署脚本（仿照 GitHub Action 的 orion-client-deploy）
#
# 默认行为：
# 1) 构建 release 二进制
# 2) 组装本地 artifacts
# 3) 部署到 /home/orion/orion-runner
# 4) 安装/重载 systemd 单元并启动服务
#
# 用法：
#   ./deploy-local.sh
# ==============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
ARTIFACT_DIR="/tmp/orion-local-deploy/artifacts"
RUNTIME_DIR="/home/orion/orion-runner"
UNIT_PATH="/etc/systemd/system/orion-runner.service"

if [[ "$EUID" -ne 0 ]]; then
    echo "请使用 root 运行：sudo ./orion/deploy-local.sh"
    exit 1
fi

prepare_runtime_dirs() {
    mkdir -p /data/scorpio/store
    mkdir -p /data/scorpio/antares/upper
    mkdir -p /data/scorpio/antares/cl
    mkdir -p /data/scorpio/antares/mnt
    mkdir -p /workspace/mount
}

restart_service() {
    echo "==> 重载并重启 systemd 服务"
    systemctl daemon-reload
    systemctl enable orion-runner.service >/dev/null 2>&1 || true

    systemctl restart orion-runner.service
    sleep 2
    systemctl --no-pager --full status orion-runner.service | head -n 30
}

echo "==> [1/4] 构建 Orion release"
cd "$REPO_ROOT"
cargo build --release -p orion --bin orion

echo "==> [2/4] 组装本地 artifacts"
rm -rf /tmp/orion-local-deploy
mkdir -p "$ARTIFACT_DIR"

cp "$REPO_ROOT/target/release/orion" "$ARTIFACT_DIR/orion"
cp "$REPO_ROOT/orion/runner-config/.env.prod" "$ARTIFACT_DIR/.env"
cp "$REPO_ROOT/orion/runner-config/scorpio.toml" "$ARTIFACT_DIR/scorpio.toml"
cp "$REPO_ROOT/orion/runner-config/cleanup.sh" "$ARTIFACT_DIR/cleanup.sh"
cp "$REPO_ROOT/orion/systemd/orion-runner.service" "$ARTIFACT_DIR/orion-runner.service"
chmod +x "$ARTIFACT_DIR/orion" "$ARTIFACT_DIR/cleanup.sh"

ls -la "$ARTIFACT_DIR"

echo "==> [3/4] 部署到本地运行目录"
mkdir -p "$RUNTIME_DIR"
cp -f "$ARTIFACT_DIR/orion" "$RUNTIME_DIR/orion"
cp -f "$ARTIFACT_DIR/.env" "$RUNTIME_DIR/.env"
cp -f "$ARTIFACT_DIR/scorpio.toml" "$RUNTIME_DIR/scorpio.toml"
cp -f "$ARTIFACT_DIR/cleanup.sh" "$RUNTIME_DIR/cleanup.sh"
chmod +x "$RUNTIME_DIR/orion" "$RUNTIME_DIR/cleanup.sh"

cp -f "$ARTIFACT_DIR/orion-runner.service" "$UNIT_PATH"
prepare_runtime_dirs

# 与 workflow 行为保持一致：每次替换二进制后重新授予能力
setcap cap_dac_read_search+ep "$RUNTIME_DIR/orion" || true
getcap "$RUNTIME_DIR/orion" || true

echo "==> [4/4] 安装并启动服务"
restart_service

echo "==> 完成"
