use crate::prelude::*;

pub fn root_partition(c: Config) -> Config {
    let root_part: String = get_value(&c, "disk.root_partition");

    if !get_value::<bool>(&c, "disk.encryption.enable") {
        return add_value(c, "disk.root_container", root_part);
    }

    let method: String = get_value(&c, "disk.encryption.method");
    if method == "password" {
        let rc = encrypt_pw();
        return add_value(c, "disk.root_container", rc);
    } else if method == "keydrive" {
        let (rc, keypart, keymount) = encrypt_kd();
        let c = add_value(c, "disk.root_container", rc);
        let c = add_value(c, "disk.keydrive.partition", keypart);
        let c = add_value(c, "disk.keydrive.mount", keymount);
        return c;
    } else {
        Error::Config("Improper encryption method supplied".into()).handle()
    }
}

fn encrypt_pw() -> String {
    "".into()
}

fn encrypt_kd() -> (String, String, String) {
    ("".into(), "".into(), "".into())
}

fn password_encrypt(target: &str, mount: &str) {
    let prompt = format!("Input new password for {} >>: ", target);
    let pw = rpassword::prompt_password(prompt);
    let pw = pw.expect("Failed to prompt for password");

    shrun(&ShellCommand::new("cryptsetup").pipe_string(&pw).args([
        "luksFormat",
        "-c",
        "aes-xts-plain64",
        "-h",
        "sha512",
        "-s",
        "512",
        target,
    ]));

    shrun(
        &ShellCommand::new("cryptsetup")
            .pipe_string(&pw)
            .args(["open", "--type", "luks", target, mount]),
    );
}
