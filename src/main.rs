use std::io::{self, Write};
use std::process::Command;

struct App {
    name: &'static str,
    description: &'static str,
    mandatory: bool,
    linux_cmd: &'static str,
    linux_args: Vec<&'static str>,
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

fn install_app(app: &App) {
    #[cfg(target_os = "windows")]
    let (cmd, args) = (app.windows_cmd, &app.windows_args);

    #[cfg(target_os = "linux")]
    let (cmd, args) = (app.linux_cmd, &app.linux_args);

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

fn main() {
    println!("Кроссплатформенный автоинсталлер окружения");
    println!("==========================================");

    let apps = vec![
        App {
            name: "Git",
            description: "Система контроля версий. Необходима для скачивания кода и работы с репозиториями.",
            mandatory: true,
            linux_cmd: "sudo",
            linux_args: vec!["dnf", "install", "git", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "Git.Git", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "Element",
            description: "Децентрализованный мессенджер протокола Matrix для защищенной связи.",
            mandatory: true,
            linux_cmd: "flatpak",
            linux_args: vec!["install", "flathub", "im.riot.Riot", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "Element.Element", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "CLion",
            description: "Идеальная среда для системного программирования на C и C++.",
            mandatory: false,
            linux_cmd: "flatpak",
            linux_args: vec!["install", "flathub", "com.jetbrains.CLion", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "JetBrains.CLion", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "RustRover",
            description: "Мощная среда для разработки безопасных и быстрых программ на Rust.",
            mandatory: false,
            linux_cmd: "flatpak",
            linux_args: vec!["install", "flathub", "com.jetbrains.RustRover", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "JetBrains.RustRover", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "WebStorm",
            description: "Это лучше для тех, кто хочет делать веб-сайты и веб-клиенты.",
            mandatory: false,
            linux_cmd: "flatpak",
            linux_args: vec!["install", "flathub", "com.jetbrains.WebStorm", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "JetBrains.WebStorm", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "Android Studio",
            description: "Официальная среда для разработки нативных мобильных приложений.",
            mandatory: false,
            linux_cmd: "flatpak",
            linux_args: vec!["install", "flathub", "com.google.AndroidStudio", "-y"],
            windows_cmd: "winget",
            windows_args: vec!["install", "--id", "Google.AndroidStudio", "-e", "--accept-source-agreements", "--accept-package-agreements"],
        },
        App {
            name: "Visual Studio 2022 Community",
            description: "Мощная IDE от Microsoft, отлично подходит для разработки на C++.",
            mandatory: false,
            linux_cmd: "",
            linux_args: vec![],
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

    println!("Работа инсталлера успешно завершена.");
}