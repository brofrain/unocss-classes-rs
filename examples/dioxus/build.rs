use std::process::Command;

fn main() {
    Command::new("cp")
        .args([
            "node_modules/@unocss/reset/tailwind.css",
            "public/reset-tailwind.css",
        ])
        .spawn()
        .unwrap();

    Command::new("npx").arg("unocss").spawn().unwrap();
}
