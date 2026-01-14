{
  description = "Lapce - A lightning-fast and powerful code editor";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # 从 rust-toolchain.toml 读取 Rust 工具链
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };

        # 系统依赖
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          cmake
          perl
          python3
        ];

        buildInputs = with pkgs; [
          # 核心库
          openssl
          zlib
          libgit2
          zstd
          
          # GUI 依赖 (Floem 需要)
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libxcb
          xorg.xcbutil
          xorg.xcbutilimage
          libxkbcommon
          
          # Wayland 支持
          wayland
          
          # 渲染依赖 (根据 docker-bake.hcl)
          vulkan-loader
          vulkan-headers
          # vulkan-validation-layers  # 暂时禁用，可能与系统驱动冲突
          
          # OpenGL/EGL 支持 (wgpu 需要)
          libGL
          libglvnd  # OpenGL 供应商中立调度库
          mesa      # 提供 EGL 和 OpenGL 实现
          
          # 字体相关
          fontconfig
          freetype
          
          # 其他依赖
          expat
          
        ] ++ lib.optionals stdenv.isLinux [
          # Linux 特定依赖
          alsa-lib
          atk
          cairo
          gdk-pixbuf
          glib
          gtk3
          pango
          dbus
        ] ++ lib.optionals stdenv.isDarwin [
          # macOS 特定依赖
          darwin.apple_sdk.frameworks.AppKit
          darwin.apple_sdk.frameworks.CoreGraphics
          darwin.apple_sdk.frameworks.CoreServices
          darwin.apple_sdk.frameworks.Foundation
          darwin.apple_sdk.frameworks.Metal
          darwin.apple_sdk.frameworks.QuartzCore
          darwin.apple_sdk.frameworks.Security
        ];

        # 运行时库路径
        libraryPath = with pkgs; lib.makeLibraryPath ([
          vulkan-loader
          libGL
          libglvnd
          mesa
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          libgit2
          openssl
          zlib
        ] ++ lib.optionals stdenv.isLinux [
          stdenv.cc.cc.lib
        ]);

      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          shellHook = ''
            # 设置环境变量
            export LD_LIBRARY_PATH="${libraryPath}:$LD_LIBRARY_PATH"
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.fontconfig.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
            
            # 图形后端配置
            # 让 wgpu 自动选择最佳后端（Vulkan/OpenGL/GLES）
            # 不设置 WGPU_BACKEND，让它自动检测
            
            # 确保 EGL/OpenGL 可以被找到
            export __EGL_VENDOR_LIBRARY_DIRS="${pkgs.mesa.drivers}/share/glvnd/egl_vendor.d"
            
            # 如果在 Wayland 下遇到问题，可以尝试强制使用 X11
            # export WAYLAND_DISPLAY=""
            # export GDK_BACKEND=x11
            
            # Rust 配置
            export RUST_BACKTRACE=1
            export CARGO_TARGET_DIR="target"
            
            # libgit2 配置 (使用系统 libgit2)
            export LIBGIT2_SYS_USE_PKG_CONFIG=1
            
            echo "🚀 Lapce development environment loaded!"
            echo "📦 Rust version: $(rustc --version)"
            echo "🔧 Cargo version: $(cargo --version)"
            echo "🎨 Graphics: Auto-detecting backend (Vulkan/OpenGL)"
          '';

          # 防止 Cargo 使用 vendored 依赖时出现问题
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.libclang.version}/include";
        };

        # 注意：由于项目使用了多个 Git 依赖，构建 Nix 包需要正确配置所有哈希值
        # 如需构建包，请参考 https://nixos.org/manual/nixpkgs/stable/#using-buildrustpackage
        # 暂时只提供开发环境配置
      }
    );
}
