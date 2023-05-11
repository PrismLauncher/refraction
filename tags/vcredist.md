---
aliases: ['msvc']
embed:
  title: vcredist is required for Prism to run Windows
  color: pink
---

Like most apps on Windows, you have to install vcredist for Prism to run. Depending on what version of Prism you are using, you may need a different version.

You need:

- [vcredist 2022 x64](https://aka.ms/vs/17/release/vc_redist.x64.exe) if you're using PrismLauncher-Windows-MSVC (the recommended version for Windows 10 64 bit/Windows 11).
- [vcredist 2022 x86](https://aka.ms/vs/17/release/vc_redist.x86.exe) if you're using PrismLauncher-Windows-MSVC-Legacy (the recommended version for Windows 7/8.1 and Windows 10 32 bit).
- [vcredist 2022 arm64](https://aka.ms/vs/17/release/vc_redist.arm64.exe) if you're using PrismLauncher-Windows-MSVC-arm64 (the recommended version for Windows 10/11 on ARM).

See the [wiki page](https://prismlauncher.org/wiki/overview/frequent-issues/#%22msvcp140_2.dll-was-not-found%22) on Prism's website for more information.
