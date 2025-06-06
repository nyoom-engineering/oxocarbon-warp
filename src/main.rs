use std::{
    env,
    fs,
    io::{self, Read},
    process,
};

fn handle_error<T>(result: Result<T, impl std::fmt::Display>, message: &str) -> T {
    result.unwrap_or_else(|e| {
        eprintln!("{}: {}", message, e);
        process::exit(1);
    })
}

fn main() {
    let mut args = env::args().skip(1);
    let mut oled = false;
    let mut input_src = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--oled" => oled = true,
            other if input_src.is_none() => input_src = Some(other.to_string()),
            _ => {}
        }
    }
    let input_src = input_src.unwrap_or_else(|| "-".into());

    let mut toml_buf = String::new();
    if input_src == "-" {
        handle_error(
            io::stdin().read_to_string(&mut toml_buf),
            "Failed to read from stdin"
        );
    } else {
        toml_buf = handle_error(
            fs::read_to_string(&input_src),
            &format!("Failed to read '{}'", input_src)
        );
    }

    if oled {
        // Perform chained replacements for OLED
        let replacements = [
            ("#161616", "#000000"),
            ("#1b1b1b", "#0b0b0b"),
            ("#1e1e1e", "#0b0b0b"),
            ("#212121", "#0f0f0f"),
            ("#262626", "#161616"),
            ("#393939", "#262626"),
            ("#525252", "#393939"),
        ];

        for &(from, to) in &replacements {
            toml_buf = toml_buf.replace(from, to);
        }

        // Override name
        toml_buf = toml_buf
            .lines()
            .map(|line| {
                if line.trim_start().starts_with("name") {
                    "name = \"Oxocarbon Oled\"".to_string()
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
    }

    let value: toml::Value = handle_error(
        toml::from_str(&toml_buf),
        &format!("TOML parse error ({})", input_src)
    );

    let stdout = io::stdout();
    let handle = stdout.lock();
    handle_error(
        serde_yaml::to_writer(handle, &value),
        "Failed to write YAML"
    );
}
